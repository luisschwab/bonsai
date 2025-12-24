use core::fmt::Display;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use bdk_floresta::ConnectionKind;
use bdk_floresta::FlorestaNode;
use bdk_floresta::PeerInfo;
use bdk_floresta::PeerStatus;
use bdk_floresta::TransportProtocol;
use bdk_floresta::rustreexo::accumulator::stump::Stump;
use bitcoin::p2p::ServiceFlags;
use regex::Regex;
use tokio::sync::RwLock;

use crate::node::message::NodeMessage;

#[derive(Clone, Default, Debug)]
pub(crate) enum NodeImpl {
    Btcd,
    Core,
    Floresta,
    Utreexod,
    Knots,
    #[default]
    Unknown,
}

impl Display for NodeImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Btcd => write!(f, "btcd"),
            Self::Core => write!(f, "Bitcoin Core"),
            Self::Floresta => write!(f, "Floresta"),
            Self::Utreexod => write!(f, "Utreexod"),
            Self::Knots => write!(f, "Bitcoin Knots"),
            Self::Unknown => writeln!(f, "Unknown"),
        }
    }
}

#[allow(unused)]
#[derive(Clone)]
pub(crate) struct PeerInformation {
    pub(crate) socket: SocketAddr,
    pub(crate) services: ServiceFlags,
    pub(crate) user_agent: String,
    pub(crate) node_impl: NodeImpl,
    pub(crate) initial_height: u32,
    pub(crate) peer_status: PeerStatus,
    pub(crate) connection_kind: ConnectionKind,
    pub(crate) transport_protocol: TransportProtocol,
}

#[derive(Clone)]
pub(crate) struct NodeStatistics {
    pub(crate) in_ibd: bool,
    pub(crate) headers: u32,
    pub(crate) blocks: u32,
    pub(crate) accumulator: Stump,
    pub(crate) accumulator_qr_data: Option<String>,
    pub(crate) user_agent: String,
    pub(crate) peer_informations: Vec<PeerInformation>,
    pub(crate) uptime: Duration,
}

fn encode_stump(stump: &Stump) -> String {
    let mut buffer = Vec::new();
    if let Err(e) = stump.serialize(&mut buffer) {
        tracing::error!("Failed to serialize stump: {}", e);
        return String::new();
    }
    hex::encode(buffer)
}

fn regex_user_agent(user_agent: &str) -> NodeImpl {
    if Regex::new(r"Satoshi.*Knots").unwrap().is_match(user_agent) {
        NodeImpl::Knots
    } else if Regex::new(r"Satoshi").unwrap().is_match(user_agent) {
        NodeImpl::Core
    } else if user_agent.contains("btcd") {
        NodeImpl::Btcd
    } else if user_agent.contains("utreexod") {
        NodeImpl::Utreexod
    } else if user_agent.contains("floresta") {
        NodeImpl::Floresta
    } else {
        NodeImpl::Unknown
    }
}

fn process_peer_infos(peer_infos: Vec<PeerInfo>) -> Vec<PeerInformation> {
    let mut peer_informations: Vec<PeerInformation> = Vec::new();
    for peer_info in peer_infos {
        let peer_information = PeerInformation {
            socket: peer_info.address,
            services: peer_info.services,
            node_impl: regex_user_agent(&peer_info.user_agent),
            user_agent: peer_info.user_agent,
            initial_height: peer_info.initial_height,
            peer_status: peer_info.state,
            connection_kind: peer_info.kind,
            transport_protocol: peer_info.transport_protocol,
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
        let user_agent = node_handle.get_config().await.unwrap().user_agent;
        let uptime = start_time
            .map(|t| t.elapsed())
            .unwrap_or(Duration::from_secs(0));
        let peer_infos_raw = node_handle.get_peer_info().await.unwrap_or_default();
        let peer_informations = process_peer_infos(peer_infos_raw);

        let encoded_stump = encode_stump(&accumulator);
        let accumulator_qr_data = if !encoded_stump.is_empty() {
            Some(encoded_stump)
        } else {
            None
        };

        Ok(NodeStatistics {
            in_ibd,
            headers,
            blocks,
            accumulator,
            user_agent,
            accumulator_qr_data,
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
