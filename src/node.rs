use std::sync::Arc;
use std::sync::OnceLock;

use bdk_floresta::PeerInfo;
use bdk_floresta::builder::FlorestaBuilder;
use bdk_floresta::rustreexo::accumulator::stump::Stump;
use bdk_floresta::{FlorestaNode, UtreexoNodeConfig};
use bdk_wallet::bitcoin::Network;
use iced::futures::SinkExt;
use iced::widget::{column, text};
use iced::{Element, Font, Subscription};
use tokio::sync::RwLock;

pub const DATA_DIR: &str = "./data/";
pub const NETWORK: Network = Network::Signet;

static RUNTIME_HANDLE: OnceLock<tokio::runtime::Handle> = OnceLock::new();

pub fn set_runtime_handle(handle: tokio::runtime::Handle) {
    RUNTIME_HANDLE
        .set(handle)
        .expect("Runtime handle already set");
}

fn get_runtime_handle() -> &'static tokio::runtime::Handle {
    RUNTIME_HANDLE.get().expect("Runtime handle not set")
}

#[derive(Clone)]
pub struct NodeStats {
    pub in_ibd: bool,
    pub chain_height: u32,
    pub validated_height: u32,
    pub accumulator: Stump,
    pub peer_info: Vec<PeerInfo>,
}

#[derive(Default)]
pub struct Node {
    handle: Option<Arc<RwLock<FlorestaNode>>>,
    stats: Option<NodeStats>,
    error: Option<String>,
}

impl Node {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::NodeReady(handle) => {
                self.handle = Some(handle);
                self.error = None;
            }
            Message::NodeStats(stats) => {
                self.stats = Some(stats);
                self.error = None;
            }
            Message::ErrorOccurred(err) => {
                self.error = Some(err);
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let status_text = if let Some(err) = &self.error {
            format!("Error: {}", err)
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
            text(format!("CHAIN HEIGHT: {}", chain_height)).font(Font::MONOSPACE),
            text(format!("VALIDATED HEIGHT: {}", validated_height)).font(Font::MONOSPACE),
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

    pub fn subscription(&self) -> Subscription<Message> {
        if let Some(node_handle) = &self.handle {
            let node_handle = node_handle.clone();
            let rt_handle = get_runtime_handle().clone();

            Subscription::run_with_id(
                "node_stats",
                iced::stream::channel(100, move |mut output| async move {
                    let _ = rt_handle
                        .spawn(async move {
                            loop {
                                tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
                                let msg = fetch_stats(node_handle.clone()).await;
                                if output.send(msg).await.is_err() {
                                    break;
                                }
                            }
                        })
                        .await;
                }),
            )
        } else {
            Subscription::none()
        }
    }
}

#[derive(Clone)]
pub enum Message {
    NodeReady(Arc<RwLock<FlorestaNode>>),
    NodeStats(NodeStats),
    ErrorOccurred(String),
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::NodeReady(_) => write!(f, "NodeReady"),
            Message::NodeStats(_) => write!(f, "NodeStats"),
            Message::ErrorOccurred(err) => f.debug_tuple("ErrorOccurred").field(err).finish(),
        }
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

pub(crate) async fn fetch_stats(handle: Arc<RwLock<FlorestaNode>>) -> Message {
    let result = async {
        let node_handle = handle.read().await;

        Ok(NodeStats {
            in_ibd: node_handle.in_ibd().unwrap(),
            chain_height: node_handle.get_height().unwrap_or(0),
            validated_height: node_handle.get_validation_height().unwrap_or(0),
            accumulator: node_handle.get_accumulator().unwrap(),
            peer_info: node_handle.get_peer_info().await.unwrap_or_default(),
        })
    }
    .await;

    match result {
        Ok(stats) => Message::NodeStats(stats),
        Err(e) => Message::ErrorOccurred(e),
    }
}
