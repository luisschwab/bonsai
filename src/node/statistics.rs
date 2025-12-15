use std::sync::Arc;

use bdk_floresta::FlorestaNode;
use bdk_floresta::PeerInfo;
use bdk_floresta::rustreexo::accumulator::stump::Stump;
use tokio::sync::RwLock;

use crate::node::message::NodeMessage;

#[derive(Clone)]
pub struct NodeStatistics {
    pub in_ibd: bool,
    pub chain_height: u32,
    pub validated_height: u32,
    pub accumulator: Stump,
    pub peer_info: Vec<PeerInfo>,
}

pub(crate) async fn fetch_stats(handle: Arc<RwLock<FlorestaNode>>) -> NodeMessage {
    let result = async {
        let node_handle = handle.read().await;

        Ok(NodeStatistics {
            in_ibd: node_handle.in_ibd().unwrap(),
            chain_height: node_handle.get_height().unwrap_or(0),
            validated_height: node_handle.get_validation_height().unwrap_or(0),
            accumulator: node_handle.get_accumulator().unwrap(),
            peer_info: node_handle.get_peer_info().await.unwrap_or_default(),
        })
    }
    .await;

    match result {
        Ok(stats) => NodeMessage::Statistics(stats),
        Err(e) => NodeMessage::Error(e),
    }
}
