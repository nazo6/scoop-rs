#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse manifest json: {0}")]
    ManifestParse(#[source] serde_json::Error),
    #[error("Failed to parse {0} json: {1}")]
    JsonParse(&'static str, #[source] serde_json::Error),
    #[error("Invalid state: {0}")]
    InvalidState(String),
    #[error("{0}: {1}")]
    WithContext(String, Box<Error>),
}

pub(crate) trait Context<T> {
    fn context(self, context: impl Into<String>) -> Result<T>;
    fn with_context(self, context_fn: impl FnOnce() -> String) -> Result<T>;
}

impl<T, E> Context<T> for std::result::Result<T, E>
where
    E: Into<Error>,
{
    fn context(self, context: impl Into<String>) -> Result<T> {
        self.map_err(|e| Error::WithContext(context.into(), Box::new(e.into())))
    }
    fn with_context(self, context_fn: impl FnOnce() -> String) -> Result<T> {
        self.map_err(|e| Error::WithContext(context_fn(), Box::new(e.into())))
    }
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
