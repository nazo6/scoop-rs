mod schema;
use std::str::FromStr;

pub use schema::arch::*;
pub use schema::*;

use crate::{error::Result, Context as _};

impl Manifest {
    pub async fn from_path(path: &std::path::Path) -> Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        Manifest::from_str(&content).with_context(|| format!("In file `{}`", path.display()))
    }
}

impl FromStr for Manifest {
    type Err = crate::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(crate::Error::ManifestParse)
    }
}
