use core::fmt::Debug;
use std::fmt::write;
use std::net::SocketAddr;
use std::sync::Arc;

use bdk_floresta::FlorestaNode;
use bitcoin::Block;
use tokio::sync::RwLock;

use crate::node::error::BonsaiNodeError;
use crate::node::statistics::NodeStatistics;

#[derive(Clone)]
pub(crate) enum NodeMessage {
    #[allow(unused)]
    Start,
    Restart,
    Starting,
    Running(Arc<RwLock<FlorestaNode>>),
    Shutdown,
    ShuttingDown,
    ShutdownComplete,
    Tick,
    GetStatistics,
    Statistics(NodeStatistics),
    ClearLogs,
    AddPeerInputChanged(String),
    AddPeer,
    PeerConnected(String),
    DisconnectPeer(SocketAddr),
    PeerDisconnected(SocketAddr),
    CopyAccumulatorData,
    BlockHeightInputChanged(String),
    BlockExplorerHeightUpdate(u32),
    FetchBlock(u32),
    BlockFetched(Option<Block>),
    ToggleTransactionExpandedIdx(usize),
    Error(BonsaiNodeError),
}

impl Debug for NodeMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "Start Node"),
            Self::Restart => write!(f, "Restart"),
            Self::Starting => write!(f, "Node Starting"),
            Self::Running(_) => write!(f, "Node Running"),
            Self::Shutdown => write!(f, "Stop Node"),
            Self::ShuttingDown => write!(f, "Node Shutting Down..."),
            Self::ShutdownComplete => write!(f, "Shutdown Complete"),
            Self::Tick => write!(f, "Tick"),
            Self::GetStatistics => write!(f, "Get Stats"),
            Self::Statistics(_) => write!(f, "Node Statistics"),
            Self::ClearLogs => write!(f, "Clear Logs"),
            Self::AddPeerInputChanged(peer) => write!(f, "AddPeerInputChanged({peer})"),
            Self::AddPeer => write!(f, "AddPeer"),
            Self::PeerConnected(peer) => write!(f, "PeerConnected({peer})"),
            Self::DisconnectPeer(peer) => write!(f, "RemovePeer({peer})"),
            Self::PeerDisconnected(peer) => write!(f, "PeerRemoved({peer})"),
            Self::CopyAccumulatorData => write!(f, "CopyAccumulatorData"),
            Self::BlockHeightInputChanged(input) => write!(f, "BlockHeightInputChanged({input})"),
            Self::BlockExplorerHeightUpdate(height) => {
                write!(f, "BlockExplorerHeightUpdate({height})")
            }
            Self::FetchBlock(height) => write!(f, "FetchBlock({height})"),
            Self::BlockFetched(block) => match block {
                Some(block) => write!(f, "BlockFetched({})", block.header.block_hash()),
                None => write!(f, "BlockFetched(Missing)"),
            },
            Self::ToggleTransactionExpandedIdx(idx) => {
                write!(f, "ToggleTransactionExpandedIdx({idx})")
            }
            Self::Error(_) => write!(f, "Node Error"),
        }
    }
}
