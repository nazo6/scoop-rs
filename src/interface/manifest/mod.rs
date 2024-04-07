mod schema;
use anyhow::Context as _;
pub use schema::*;

impl Manifest {
    pub fn from_str(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
    pub async fn from_path(path: &std::path::Path) -> Result<Self, anyhow::Error> {
        let content = tokio::fs::read_to_string(path)
            .await
            .context("Failed to read manifest file")?;
        Ok(Manifest::from_str(&content).context("Failed to parse manifest")?)
    }
}
