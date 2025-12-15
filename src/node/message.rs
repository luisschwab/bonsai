use core::fmt::Debug;
use std::sync::Arc;

use bdk_floresta::FlorestaNode;
use tokio::sync::RwLock;

use crate::node::error::BonsaiNodeError;
use crate::node::statistics::NodeStatistics;

#[derive(Clone)]
pub(crate) enum NodeMessage {
    #[allow(unused)]
    Starting,
    Running(Arc<RwLock<FlorestaNode>>),
    ShuttingDown,
    ShutdownComplete,
    GetStatistics,
    Statistics(NodeStatistics),
    Error(BonsaiNodeError),
}

impl Debug for NodeMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Starting => write!(f, "Node Starting"),
            Self::Running(_) => write!(f, "Node Running"),
            Self::ShuttingDown => write!(f, "Node Shutting Down..."),
            Self::ShutdownComplete => write!(f, "Shutdown Complete"),
            Self::GetStatistics => write!(f, "Get Stats"),
            Self::Statistics(_) => write!(f, "Node Statistics"),
            Self::Error(_) => write!(f, "Node Error"),
        }
    }
}
