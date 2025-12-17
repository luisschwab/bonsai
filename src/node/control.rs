use core::fmt::Display;

use std::sync::Arc;
use std::time::{Duration, Instant};

use bdk_floresta::builder::FlorestaBuilder;
use bdk_floresta::{FlorestaNode, UtreexoNodeConfig};
use bdk_wallet::bitcoin::Network;
use iced::{Element, Subscription, Task};
use tokio::runtime::Handle;
use tokio::sync::RwLock;

use crate::Tab;
use crate::node::error::BonsaiNodeError;
use crate::node::logger::LogCapture;
use crate::node::message::NodeMessage;
use crate::node::statistics::NodeStatistics;
use crate::node::statistics::fetch_stats;

pub const DATA_DIR: &str = "./data/";
pub const NETWORK: Network = Network::Signet;
pub const FETCH_STATISTICS_TIME: u64 = 1;

#[derive(Default)]
pub(crate) enum NodeStatus {
    #[default]
    Inactive,
    Starting,
    Running,
    ShuttingDown,
    Failed(BonsaiNodeError),
}

impl Display for NodeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Inactive => write!(f, "INACTIVE"),
            Self::Starting => write!(f, "STARTING"),
            Self::Running => write!(f, "RUNNING"),
            Self::ShuttingDown => write!(f, "SHUTTING DOWN"),
            Self::Failed(_) => write!(f, "FAILED"),
        }
    }
}

#[derive(Default)]
pub(crate) struct Node {
    pub(crate) handle: Option<Arc<RwLock<FlorestaNode>>>,
    pub(crate) status: NodeStatus,
    pub(crate) statistics: Option<NodeStatistics>,
    pub(crate) subscription_active: bool,
    pub(crate) is_shutting_down: bool,
    pub(crate) log_capture: LogCapture,
    pub(crate) start_time: Option<Instant>,
}

impl Node {
    pub fn update(&mut self, message: NodeMessage) -> Task<NodeMessage> {
        match message {
            NodeMessage::Start => {
                self.status = NodeStatus::Starting;
                Task::perform(start_node(), |res| match res {
                    Ok(handle) => NodeMessage::Running(handle),
                    Err(e) => NodeMessage::Error(BonsaiNodeError::from(e)),
                })
            }
            NodeMessage::Starting => {
                self.status = NodeStatus::Starting;
                self.subscription_active = false;
                Task::none()
            }
            NodeMessage::Running(handle) => {
                self.handle = Some(handle);
                self.status = NodeStatus::Running;
                self.subscription_active = true;
                self.is_shutting_down = false;
                self.start_time = Some(Instant::now());
                Task::none()
            }
            NodeMessage::Shutdown => {
                self.status = NodeStatus::ShuttingDown;
                self.subscription_active = false;
                self.is_shutting_down = true;
                self.start_time = None;

                if let Some(node_handle) = self.handle.take() {
                    let rt_handle = Handle::current();

                    Task::future(async move {
                        let result = rt_handle
                            .spawn(async move { stop_node(node_handle).await })
                            .await;

                        match result {
                            Ok(Ok(_)) => NodeMessage::ShutdownComplete,
                            Ok(Err(e)) => NodeMessage::Error(BonsaiNodeError::from(e)),
                            Err(e) => NodeMessage::Error(BonsaiNodeError::Generic(e.to_string())),
                        }
                    })
                } else {
                    Task::done(NodeMessage::ShutdownComplete)
                }
            }
            NodeMessage::ShuttingDown => {
                self.status = NodeStatus::ShuttingDown;
                self.subscription_active = false;
                self.is_shutting_down = true;

                if let Some(stats) = &mut self.statistics {
                    stats.peer_info.clear();
                }

                Task::none()
            }
            NodeMessage::ShutdownComplete => {
                self.status = NodeStatus::Inactive;
                self.subscription_active = false;
                self.is_shutting_down = false;

                if let Some(stats) = &mut self.statistics {
                    stats.peer_info.clear();
                }

                Task::none()
            }
            NodeMessage::Tick => Task::none(),
            NodeMessage::Statistics(stats) => {
                if !self.is_shutting_down {
                    self.statistics = Some(stats);
                    // Don't clear error status here
                }

                Task::none()
            }
            NodeMessage::Error(err) => {
                self.status = NodeStatus::Failed(err);
                self.subscription_active = false;
                Task::none()
            }
            NodeMessage::GetStatistics => {
                if self.subscription_active {
                    if let Some(handle) = &self.handle {
                        let handle = handle.clone();
                        let rt_handle = Handle::current();
                        let start_time = self.start_time;

                        Task::future(async move {
                            rt_handle
                                .spawn(async move { fetch_stats(handle, start_time).await })
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

    pub(crate) fn subscribe(&self) -> Subscription<NodeMessage> {
        let tick_subscription =
            iced::time::every(Duration::from_millis(300)).map(|_| NodeMessage::Tick);

        if self.subscription_active {
            Subscription::batch(vec![
                iced::time::every(Duration::from_secs(FETCH_STATISTICS_TIME))
                    .map(|_| NodeMessage::GetStatistics),
                tick_subscription,
            ])
        } else {
            tick_subscription
        }
    }

    pub fn unsubscribe(&mut self) {
        self.subscription_active = false;
    }

    pub(crate) fn view_tab(&self, tab: Tab) -> Element<'_, NodeMessage> {
        match tab {
            Tab::NodeOverview => self.view_overview(),
            Tab::NodeP2P => self.view_p2p(),
            Tab::NodeBlocks => self.view_blocks(),
            Tab::NodeUtreexo => self.view_utreexo(),
            Tab::NodeSettings => self.view_settings(),
            _ => unreachable!(),
        }
    }

    fn view_overview(&self) -> Element<'_, NodeMessage> {
        use crate::node::interface::container::overview;
        overview::view_overview(&self.status, &self.statistics, &self.log_capture)
    }

    pub(crate) fn view_p2p(&self) -> Element<'_, NodeMessage> {
        use crate::node::interface::container::p2p;
        p2p::view_p2p(&self.status, &self.statistics)
    }

    pub(crate) fn view_blocks(&self) -> Element<'_, NodeMessage> {
        unimplemented!()
    }

    pub(crate) fn view_utreexo(&self) -> Element<'_, NodeMessage> {
        unimplemented!()
    }

    pub(crate) fn view_settings(&self) -> Element<'_, NodeMessage> {
        unimplemented!()
    }
}

pub(crate) async fn start_node() -> Result<Arc<RwLock<FlorestaNode>>, String> {
    let rt_handle = Handle::current();

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
