use bdk_floresta::error::BuilderError;
use bdk_floresta::error::NodeError;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub(crate) enum BonsaiNodeError {
    #[error("Generic Error: {0}")]
    Generic(String),

    #[error(transparent)]
    NodeBuildError(#[from] BuilderError),

    #[error(transparent)]
    NodeExecError(#[from] NodeError),
}

impl From<String> for BonsaiNodeError {
    fn from(s: String) -> Self {
        BonsaiNodeError::Generic(s)
    }
}
