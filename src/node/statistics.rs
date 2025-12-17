use std::sync::Arc;
use std::time::{Duration, Instant};

use bdk_floresta::FlorestaNode;
use bdk_floresta::PeerInfo;
use bdk_floresta::rustreexo::accumulator::stump::Stump;
use tokio::sync::RwLock;

use crate::node::message::NodeMessage;

#[derive(Clone)]
pub(crate) struct NodeStatistics {
    pub(crate) in_ibd: bool,
    pub(crate) chain_height: u32,
    pub(crate) validated_height: u32,
    pub(crate) accumulator: Stump,
    pub(crate) peer_info: Vec<PeerInfo>,
    pub(crate) uptime: Duration,
}

pub(crate) async fn fetch_stats(
    node_handle: Arc<RwLock<FlorestaNode>>,
    start_time: Option<Instant>,
) -> NodeMessage {
    let result = async {
        let node_handle = node_handle.read().await;

        let uptime = start_time
            .map(|t| t.elapsed())
            .unwrap_or(Duration::from_secs(0));

        Ok(NodeStatistics {
            in_ibd: node_handle.in_ibd().unwrap(),
            chain_height: node_handle.get_height().unwrap_or(0),
            validated_height: node_handle.get_validation_height().unwrap_or(0),
            accumulator: node_handle.get_accumulator().unwrap(),
            peer_info: node_handle.get_peer_info().await.unwrap_or_default(),
            uptime,
        })
    }
    .await;

    match result {
        Ok(stats) => NodeMessage::Statistics(stats),
        Err(e) => NodeMessage::Error(e),
    }
}
