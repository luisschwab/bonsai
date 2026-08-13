use std::sync::Arc;

use bdk_floresta::error::BuilderError;
use bdk_floresta::error::NodeError;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub(crate) enum BonsaiNodeError {
    #[error("Generic Error: {0}")]
    Generic(String),

    #[error(transparent)]
    NodeBuildError(Arc<BuilderError>),

    #[error(transparent)]
    NodeExecError(Arc<NodeError>),
}

impl From<BuilderError> for BonsaiNodeError {
    fn from(error: BuilderError) -> Self {
        Self::NodeBuildError(Arc::new(error))
    }
}

impl From<NodeError> for BonsaiNodeError {
    fn from(error: NodeError) -> Self {
        Self::NodeExecError(Arc::new(error))
    }
}

impl From<String> for BonsaiNodeError {
    fn from(s: String) -> Self {
        BonsaiNodeError::Generic(s)
    }
}
