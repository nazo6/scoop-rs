use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::val::INSTALL_PATH;

use super::bucket_app::BucketApp;

pub async fn get_buckets() -> Result<Vec<Bucket>> {
    let mut buckets = Vec::new();
    let mut reader = tokio::fs::read_dir(INSTALL_PATH.clone().join("buckets")).await?;
    while let Ok(Some(entry)) = reader.next_entry().await {
        let bucket = Bucket::from_name(entry.file_name().to_str().unwrap());
        buckets.push(bucket);
    }
    Ok(buckets)
}

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
#[serde(transparent)]
pub struct Bucket {
    pub name: String,
}

impl Bucket {
    pub fn from_name(name: &str) -> Self {
        Bucket {
            name: name.to_string(),
        }
    }

    pub fn path(&self) -> std::path::PathBuf {
        INSTALL_PATH.clone().join("buckets").join(&self.name)
    }

    /// Get the git repository of the bucket
    pub fn repository(&self) -> std::result::Result<git2::Repository, git2::Error> {
        git2::Repository::open(self.path())
    }

    /// Get list of apps in the bucket
    pub async fn apps(&self) -> Result<Vec<BucketApp>> {
        let mut apps = Vec::new();
        let mut reader = tokio::fs::read_dir(self.path().join("bucket")).await?;
        while let Ok(Some(entry)) = reader.next_entry().await {
            let app = BucketApp {
                name: entry
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                bucket: self,
            };
            apps.push(app);
        }
        Ok(apps)
    }
}
