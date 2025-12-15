use std::sync::Arc;
use std::sync::OnceLock;
use std::time::Duration;

use bdk_floresta::builder::FlorestaBuilder;
use bdk_floresta::{FlorestaNode, UtreexoNodeConfig};
use bdk_wallet::bitcoin::Network;
use iced::Task;
use iced::widget::{column, text};
use iced::{Element, Font, Subscription};
use tokio::runtime::Handle;
use tokio::sync::RwLock;

use crate::node::error::BonsaiNodeError;
use crate::node::message::NodeMessage;
use crate::node::statistics::NodeStatistics;
use crate::node::statistics::fetch_stats;

pub const DATA_DIR: &str = "./data/";
pub const NETWORK: Network = Network::Signet;
pub const FETCH_STATISTICS_TIME: u64 = 5;

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
            NodeMessage::ShuttingDown => {
                self.subscription_active = false;
                self.is_shutting_down = true;
                Task::none()
            }
            NodeMessage::ShutdownComplete => {
                self.subscription_active = false;
                self.is_shutting_down = false;
                Task::none()
            }
            NodeMessage::Statistics(stats) => {
                self.stats = Some(stats);
                self.error = None;
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
        } else if self.is_shutting_down {
            "Shutting Down...".to_string()
        } else if self.handle.is_none() {
            "Starting...".to_string()
        } else {
            "Running".to_string()
        };

        let in_ibd = self.stats.as_ref().map(|s| s.in_ibd).unwrap_or(true);
        let chain_height = self.stats.as_ref().map(|s| s.chain_height).unwrap_or(0);
        let validated_height = self.stats.as_ref().map(|s| s.validated_height).unwrap_or(0);
        let accumulator = self
            .stats
            .clone()
            .map(|s| s.accumulator)
            .unwrap_or_default();
        let peer_info = self.stats.clone().map(|s| s.peer_info).unwrap_or_default();

        let mut stats_column = column![
            text("Node").size(32).font(Font::MONOSPACE),
            text("").size(10),
            text(format!("STATUS: {}", status_text)).font(Font::MONOSPACE),
            text(format!("IBD: {}", in_ibd)).font(Font::MONOSPACE),
            text(format!("HEADERS: {}", chain_height)).font(Font::MONOSPACE),
            text(format!("BLOCKS: {}", validated_height)).font(Font::MONOSPACE),
            text(format!(
                "UTREEXO ACCUMULATOR LEAVES: {}",
                accumulator.leaves
            ))
            .font(Font::MONOSPACE),
            text("UTREEXO ROOTS:").font(Font::MONOSPACE),
        ]
        .spacing(10);

        for (i, root) in accumulator.roots.iter().enumerate() {
            stats_column =
                stats_column.push(text(format!(" [{}] {}", i, root)).font(Font::MONOSPACE));
        }

        stats_column = stats_column.push(text("PEERS:").font(Font::MONOSPACE));
        for (i, peer) in peer_info.iter().enumerate() {
            stats_column = stats_column.push(
                text(format!(
                    " [{}] addr={} transport={:?} user_agent={}",
                    i, peer.address, peer.transport_protocol, peer.user_agent
                ))
                .font(Font::MONOSPACE),
            );
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
