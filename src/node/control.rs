use core::fmt::Display;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use bdk_floresta::ChainParams;
use bdk_floresta::FlorestaNode;
use bdk_floresta::UtreexoNodeConfig;
use bdk_floresta::builder::FlorestaBuilder;
use bitcoin::Network;
use iced::Element;
use iced::Subscription;
use iced::Task;
use iced::clipboard;
use iced::widget::qr_code;
use tokio::runtime::Handle;
use tokio::sync::RwLock;
use tracing::error;

use crate::Tab;
use crate::node::error::BonsaiNodeError;
use crate::node::geoip::GeoIpReader;
use crate::node::log_capture::LogCapture;
use crate::node::message::NodeMessage;
use crate::node::statistics::NodeStatistics;
use crate::node::statistics::fetch_stats;

pub const DATA_DIR: &str = "./data/";
pub const NETWORK: Network = Network::Signet;
pub const FETCH_STATISTICS_TIME: u64 = 1;

#[derive(Clone, Debug, Default)]
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
            Self::Failed(e) => write!(f, "FAILED [{}]", e),
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
    pub(crate) last_log_version: usize,
    pub(crate) start_time: Option<Instant>,
    pub(crate) peer_input: String,
    pub(crate) geoip_reader: Option<GeoIpReader>,
    //pub(crate) animation_tick: usize,
    pub(crate) accumulator_qr_data: Option<qr_code::Data>,
}

impl Node {
    pub fn update(&mut self, message: NodeMessage) -> Task<NodeMessage> {
        match message {
            // `Tick` is useful for triggering an UI re-render.
            NodeMessage::Tick => {
                //self.animation_tick = self.animation_tick.wrapping_add(1);

                let current_version = self.log_capture.version();
                if current_version != self.last_log_version {
                    self.last_log_version = current_version;
                }

                Task::none()
            }
            NodeMessage::Start => {
                self.status = NodeStatus::Starting;
                Task::perform(start_node(), |res| match res {
                    Ok(handle) => NodeMessage::Running(handle),
                    Err(e) => NodeMessage::Error(BonsaiNodeError::from(e)),
                })
            }
            NodeMessage::Restart => {
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
                            Ok(Ok(_)) => NodeMessage::Start,
                            Ok(Err(e)) => NodeMessage::Error(BonsaiNodeError::from(e)),
                            Err(e) => NodeMessage::Error(BonsaiNodeError::Generic(e.to_string())),
                        }
                    })
                } else {
                    Task::done(NodeMessage::Start)
                }
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
                //self.subscription_active = false;
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
                //self.subscription_active = false;
                self.is_shutting_down = true;

                if let Some(stats) = &mut self.statistics {
                    stats.peer_informations.clear();
                }

                Task::none()
            }
            NodeMessage::ShutdownComplete => {
                self.status = NodeStatus::Inactive;
                self.subscription_active = false;
                self.is_shutting_down = false;

                if let Some(stats) = &mut self.statistics {
                    stats.peer_informations.clear();
                }

                Task::none()
            }
            NodeMessage::Statistics(stats) => {
                if !self.is_shutting_down {
                    // Update QR data when statistics update
                    self.accumulator_qr_data = stats
                        .accumulator_qr_data
                        .as_ref()
                        .and_then(|encoded| qr_code::Data::new(encoded).ok());

                    self.statistics = Some(stats);
                }
                Task::none()
            }
            NodeMessage::Error(e) => {
                //TODO fix this
                //self.status = NodeStatus::Failed(err);
                //self.subscription_active = false;
                error!("Node Error: {e}");
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
            NodeMessage::AddPeerInputChanged(peer) => {
                self.peer_input = peer;
                Task::none()
            }
            NodeMessage::AddPeer => {
                let peer_address = self.peer_input.clone();

                if let Some(handle) = &self.handle {
                    let handle = handle.clone();
                    let rt_handle = Handle::current();

                    Task::future(async move {
                        let result = rt_handle
                            .spawn(async move {
                                // Parse the address
                                let addr: SocketAddr = match peer_address.parse() {
                                    Ok(addr) => addr,
                                    Err(e) => {
                                        return NodeMessage::Error(BonsaiNodeError::Generic(
                                            format!("Invalid peer address: {}", e),
                                        ));
                                    }
                                };

                                // Connect to the peer
                                let node = handle.read().await;
                                match node.connect_peer(&addr).await {
                                    Ok(true) => NodeMessage::PeerConnected(peer_address),
                                    Ok(false) => NodeMessage::Error(BonsaiNodeError::Generic(
                                        "Failed to connect to peer".to_string(),
                                    )),
                                    Err(e) => NodeMessage::Error(BonsaiNodeError::from(e)),
                                }
                            })
                            .await;

                        result.unwrap_or_else(|e| {
                            NodeMessage::Error(BonsaiNodeError::Generic(e.to_string()))
                        })
                    })
                } else {
                    Task::done(NodeMessage::Error(BonsaiNodeError::Generic(
                        "Node not running".to_string(),
                    )))
                }
            }
            NodeMessage::PeerConnected(_peer) => {
                // Clear the input field after button press.
                self.peer_input.clear();

                // TODO(@luisschwab): add a success notification on top-right corner.
                Task::none()
            }
            NodeMessage::DisconnectPeer(peer_address) => {
                if let Some(handle) = &self.handle {
                    let handle = handle.clone();
                    let rt_handle = Handle::current();

                    Task::future(async move {
                        let result = rt_handle
                            .spawn(async move {
                                let addr: SocketAddr = match peer_address.parse() {
                                    Ok(addr) => addr,
                                    Err(e) => {
                                        return NodeMessage::Error(BonsaiNodeError::Generic(
                                            format!("Invalid peer address: {}", e),
                                        ));
                                    }
                                };

                                let node = handle.read().await;
                                match node.disconnect_peer(&addr).await {
                                    Ok(_) => NodeMessage::PeerDisconnected(peer_address),
                                    Err(e) => NodeMessage::Error(BonsaiNodeError::from(e)),
                                }
                            })
                            .await;

                        result.unwrap_or_else(|e| {
                            NodeMessage::Error(BonsaiNodeError::Generic(e.to_string()))
                        })
                    })
                } else {
                    Task::done(NodeMessage::Error(BonsaiNodeError::Generic(
                        "Node not running".to_string(),
                    )))
                }
            }

            NodeMessage::PeerDisconnected(_peer) => {
                // TODO add success notification
                Task::none()
            }
            NodeMessage::CopyAccumulatorData => {
                if let Some(stats) = &self.statistics
                    && let Some(data) = &stats.accumulator_qr_data
                {
                    return clipboard::write(data.clone());
                }

                Task::none()
            }
        }
    }

    pub(crate) fn subscribe(&self) -> Subscription<NodeMessage> {
        let tick_subscription =
            iced::time::every(Duration::from_millis(32)).map(|_| NodeMessage::Tick);

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

    pub(crate) fn view_tab(&self, tab: Tab, animation_tick: usize) -> Element<'_, NodeMessage> {
        match tab {
            Tab::NodeOverview => self.view_overview(animation_tick),
            Tab::NodeP2P => self.view_p2p(),
            Tab::NodeBlocks => self.view_blocks(),
            Tab::NodeUtreexo => self.view_utreexo(),
            Tab::NodeSettings => self.view_settings(),
            _ => unreachable!(),
        }
    }

    fn view_overview(&self, animation_tick: usize) -> Element<'_, NodeMessage> {
        use crate::node::interface::overview::view;
        view::view_overview(
            &self.status,
            &self.statistics,
            &self.log_capture,
            animation_tick,
        )
    }

    pub(crate) fn view_p2p(&self) -> Element<'_, NodeMessage> {
        use crate::node::interface::p2p::view;
        view::view_p2p(
            &self.status,
            &self.statistics,
            &self.peer_input,
            &self.geoip_reader,
        )
    }

    pub(crate) fn view_utreexo(&self) -> Element<'_, NodeMessage> {
        use crate::node::interface::utreexo::view;
        view::view_utreexo(&self.statistics, &self.accumulator_qr_data)
    }

    pub(crate) fn view_blocks(&self) -> Element<'_, NodeMessage> {
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
                assume_utreexo: Some(ChainParams::get_assume_utreexo(NETWORK)),
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
