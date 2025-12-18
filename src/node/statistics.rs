use std::sync::Arc;
use std::time::{Duration, Instant};

use bdk_floresta::FlorestaNode;
use bdk_floresta::rustreexo::accumulator::stump::Stump;
use bdk_floresta::{ConnectionKind, PeerInfo, PeerStatus};
use tokio::sync::RwLock;

use crate::node::message::NodeMessage;

#[derive(Clone, Default, Debug)]
pub(crate) enum BitcoinImplementation {
    Core,
    Knots,
    Btcd,
    Utreexod,
    Floresta,
    #[default]
    Unknown,
}

#[derive(Clone)]
pub(crate) struct PeerInformation {
    pub(crate) address: String,
    pub(crate) services: String,
    pub(crate) user_agent: String,
    pub(crate) implementation: BitcoinImplementation,
    pub(crate) initial_height: u32,
    pub(crate) peer_status: PeerStatus,
    pub(crate) connection_kind: ConnectionKind,
    //pub(crate) transport_protocol: TransportProtocol,
}

#[derive(Clone)]
pub(crate) struct NodeStatistics {
    pub(crate) in_ibd: bool,
    pub(crate) headers: u32,
    pub(crate) blocks: u32,
    pub(crate) accumulator: Stump,
    pub(crate) peer_informations: Vec<PeerInformation>,
    pub(crate) uptime: Duration,
}

fn process_peer_infos(peer_infos: Vec<PeerInfo>) -> Vec<PeerInformation> {
    // TODO regex user agent into [`BitcoinImplementation`].

    let mut peer_informations: Vec<PeerInformation> = Vec::new();
    for peer_info in peer_infos {
        let peer_information = PeerInformation {
            address: peer_info.address,
            services: peer_info.services,
            user_agent: peer_info.user_agent,
            implementation: BitcoinImplementation::Unknown,
            initial_height: peer_info.initial_height,
            peer_status: peer_info.state,
            connection_kind: peer_info.kind,
        };
        peer_informations.push(peer_information);
    }

    peer_informations
}

pub(crate) async fn fetch_stats(
    node_handle: Arc<RwLock<FlorestaNode>>,
    start_time: Option<Instant>,
) -> NodeMessage {
    let result = async {
        let node_handle = node_handle.read().await;

        let in_ibd = node_handle.in_ibd().unwrap();
        let headers = node_handle.get_height().unwrap_or(0);
        let blocks = node_handle.get_validation_height().unwrap_or(0);
        let accumulator = node_handle.get_accumulator().unwrap();
        let uptime = start_time
            .map(|t| t.elapsed())
            .unwrap_or(Duration::from_secs(0));
        let peer_infos_raw = node_handle.get_peer_info().await.unwrap_or_default();
        let peer_informations = process_peer_infos(peer_infos_raw);

        Ok(NodeStatistics {
            in_ibd,
            headers,
            blocks,
            accumulator,
            peer_informations,
            uptime,
        })
    }
    .await;

    match result {
        Ok(stats) => NodeMessage::Statistics(stats),
        Err(e) => NodeMessage::Error(e),
    }
}
