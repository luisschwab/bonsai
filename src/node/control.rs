use std::sync::Arc;
use std::sync::OnceLock;
use std::time::Duration;

use bdk_floresta::builder::FlorestaBuilder;
use bdk_floresta::{FlorestaNode, UtreexoNodeConfig};
use bdk_wallet::bitcoin::Network;
use iced::widget::{button, column, row, text};
use iced::{Element, Subscription, Task};
use tokio::runtime::Handle;
use tokio::sync::RwLock;

use crate::node::error::BonsaiNodeError;
use crate::node::message::NodeMessage;
use crate::node::statistics::NodeStatistics;
use crate::node::statistics::fetch_stats;

pub const DATA_DIR: &str = "./data/";
pub const NETWORK: Network = Network::Signet;
pub const FETCH_STATISTICS_TIME: u64 = 1;

static RUNTIME_HANDLE: OnceLock<Handle> = OnceLock::new();

/// Set `runtime_handle` as Tokio's runtime [`Handle`].
pub fn set_runtime_handle(runtime_handle: Handle) {
    RUNTIME_HANDLE
        .set(runtime_handle)
        .expect("Tokio's runtime handle is already set");
}

/// Get Tokio's runtime [`Handle`].
pub fn get_runtime_handle() -> &'static Handle {
    RUNTIME_HANDLE
        .get()
        .expect("Tokio's runtime handle is not set")
}

#[derive(Default)]
pub(crate) struct Node {
    pub(crate) handle: Option<Arc<RwLock<FlorestaNode>>>,
    pub(crate) stats: Option<NodeStatistics>,
    pub(crate) error: Option<BonsaiNodeError>,
    subscription_active: bool,
    is_shutting_down: bool,
}

impl Node {
    pub fn update(&mut self, message: NodeMessage) -> Task<NodeMessage> {
        match message {
            NodeMessage::Start => Task::perform(start_node(), |res| match res {
                Ok(handle) => NodeMessage::Running(handle),
                Err(e) => NodeMessage::Error(BonsaiNodeError::from(e)),
            }),
            NodeMessage::Starting => {
                self.subscription_active = false;
                Task::none()
            }
            NodeMessage::Running(handle) => {
                self.handle = Some(handle);
                self.error = None;
                self.subscription_active = true;
                Task::none()
            }
            NodeMessage::Shutdown => {
                // Stop the node without closing the window
                self.subscription_active = false;
                self.is_shutting_down = true;

                if let Some(node_handle) = self.handle.take() {
                    let rt_handle = get_runtime_handle().clone();

                    Task::future(async move {
                        eprintln!("Stopping node...");
                        let result = rt_handle
                            .spawn(async move { stop_node(node_handle).await })
                            .await;

                        match result {
                            Ok(Ok(_)) => {
                                eprintln!("Node stopped successfully");
                                NodeMessage::ShutdownComplete
                            }
                            Ok(Err(e)) => {
                                eprintln!("Error stopping node: {}", e);
                                NodeMessage::Error(BonsaiNodeError::from(e))
                            }
                            Err(e) => {
                                eprintln!("Task error: {}", e);
                                NodeMessage::Error(BonsaiNodeError::Generic(e.to_string()))
                            }
                        }
                    })
                } else {
                    Task::done(NodeMessage::ShutdownComplete)
                }
            }
            NodeMessage::ShuttingDown => {
                self.subscription_active = false;
                self.is_shutting_down = true;

                if let Some(stats) = &mut self.stats {
                    stats.peer_info.clear();
                }

                Task::none()
            }
            NodeMessage::ShutdownComplete => {
                self.subscription_active = false;
                self.is_shutting_down = false;

                if let Some(stats) = &mut self.stats {
                    stats.peer_info.clear();
                }

                Task::none()
            }
            NodeMessage::Statistics(stats) => {
                if !self.is_shutting_down {
                    self.stats = Some(stats);
                    self.error = None;
                }

                Task::none()
            }
            NodeMessage::Error(err) => {
                self.error = Some(err);
                self.subscription_active = false;
                Task::none()
            }
            NodeMessage::GetStatistics => {
                if self.subscription_active {
                    if let Some(handle) = &self.handle {
                        let handle = handle.clone();
                        let rt_handle = get_runtime_handle().clone();

                        Task::future(async move {
                            rt_handle
                                .spawn(async move { fetch_stats(handle).await })
                                .await
                                .unwrap_or_else(|_| {
                                    NodeMessage::Error(BonsaiNodeError::Generic(
                                        "Failed to fetch stats".to_string(),
                                    ))
                                })
                        })
                    } else {
                        Task::none()
                    }
                } else {
                    Task::none()
                }
            }
        }
    }

    pub fn subscribe(&self) -> Subscription<NodeMessage> {
        if self.subscription_active {
            iced::time::every(Duration::from_secs(FETCH_STATISTICS_TIME))
                .map(|_| NodeMessage::GetStatistics)
        } else {
            Subscription::none()
        }
    }

    pub fn unsubscribe(&mut self) {
        self.subscription_active = false;
    }

    pub fn view(&self) -> Element<'_, NodeMessage> {
        let status_text = if let Some(err) = &self.error {
            format!("Error: {}", err)
        } else {
            match (
                self.handle.is_some(),
                self.subscription_active,
                self.is_shutting_down,
            ) {
                (_, _, true) => "Shutting Down...".to_string(),
                (true, true, false) => "Running".to_string(),
                (true, false, false) => "Starting...".to_string(),
                (false, _, false) => "Stopped".to_string(),
            }
        };

        let control_buttons = if self.handle.is_some() && !self.is_shutting_down {
            // Node is running, show stop button
            row![
                button(text("Stop Node"))
                    .on_press(NodeMessage::Shutdown)
                    .style(button::danger)
            ]
        } else if self.handle.is_none() && !self.is_shutting_down {
            // Node is stopped, show start button
            row![
                button(text("Start Node"))
                    .on_press(NodeMessage::Start)
                    .style(button::success)
            ]
        } else {
            // Node is shutting down, no buttons
            row![]
        };

        let in_ibd = self.stats.as_ref().map(|s| s.in_ibd).unwrap_or(true);
        let chain_height = self.stats.as_ref().map(|s| s.chain_height).unwrap_or(0);
        let validated_height = self.stats.as_ref().map(|s| s.validated_height).unwrap_or(0);
        let accumulator = self
            .stats
            .clone()
            .map(|s| s.accumulator)
            .unwrap_or_default();
        let peer_info =
            if self.handle.is_some() && self.subscription_active && !self.is_shutting_down {
                self.stats
                    .as_ref()
                    .map(|s| s.peer_info.clone())
                    .unwrap_or_default()
            } else {
                Vec::new()
            };

        let mut stats_column = column![
            text("Node").size(32),
            text("").size(10),
            control_buttons,
            text("").size(10),
            text(format!("STATUS: {}", status_text)),
            text(format!("IBD: {}", in_ibd)),
            text(format!("HEADERS: {}", chain_height)),
            text(format!("BLOCKS: {}", validated_height)),
            text(format!(
                "UTREEXO ACCUMULATOR LEAVES: {}",
                accumulator.leaves
            )),
            text("UTREEXO ROOTS:"),
        ]
        .spacing(10);

        for (i, root) in accumulator.roots.iter().enumerate() {
            stats_column = stats_column.push(text(format!(" [{}] {}", i, root)));
        }

        stats_column = stats_column.push(text("PEERS:"));
        for (i, peer) in peer_info.iter().enumerate() {
            stats_column = stats_column.push(text(format!(
                " [{}] addr={} transport={:?} user_agent={}",
                i, peer.address, peer.transport_protocol, peer.user_agent
            )));
        }

        stats_column.into()
    }
}

pub(crate) async fn start_node() -> Result<Arc<RwLock<FlorestaNode>>, String> {
    let rt_handle = get_runtime_handle();

    rt_handle
        .spawn(async {
            let config = UtreexoNodeConfig {
                network: NETWORK,
                datadir: format!("{}{}", DATA_DIR, NETWORK),
                ..Default::default()
            };

            let node = FlorestaBuilder::new()
                .with_config(config)
                .build()
                .await
                .map_err(|e| e.to_string())?;

            Ok(Arc::new(RwLock::new(node)))
        })
        .await
        .map_err(|e| e.to_string())?
}

pub(crate) async fn stop_node(handle: Arc<RwLock<FlorestaNode>>) -> Result<(), String> {
    match Arc::try_unwrap(handle) {
        Ok(lock) => {
            let node = lock.into_inner();
            node.shutdown().await.map_err(|e| e.to_string())
        }
        Err(arc) => {
            let count = Arc::strong_count(&arc);
            Err(format!("Cannot shutdown: {} references remain", count))
        }
    }
}
