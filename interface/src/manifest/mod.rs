mod schema;
pub use schema::*;

use crate::error::Result;

impl Manifest {
    pub fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
    pub async fn from_path(path: &std::path::Path) -> Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        Ok(Manifest::from_str(&content)?)
    }
}
