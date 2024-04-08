#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid state: {0}")]
    InvalidState(String),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
