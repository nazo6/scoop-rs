use std::collections::HashSet;

use async_walkdir::{Filtering, WalkDir};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::utils::get_stem;
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
    pub async fn apps(&self) -> Result<HashSet<BucketApp>> {
        let mut apps = HashSet::new();

        let mut entries = WalkDir::new(self.path()).filter(|entry| async move {
            if let Some(true) = entry
                .path()
                .file_name()
                .map(|f| f.to_string_lossy().starts_with('.'))
            {
                return Filtering::IgnoreDir;
            }
            Filtering::Continue
        });

        loop {
            match entries.next().await {
                Some(Ok(entry)) => {
                    if let Ok(true) = entry.file_type().await.map(|f| f.is_file()) {
                        let name = entry.file_name();
                        let name = name.to_string_lossy();
                        let (name, ext) = get_stem(&name);
                        if ext == Some("json") {
                            apps.insert(BucketApp {
                                name: name.to_string(),
                                metadata_path: entry.path().to_path_buf(),
                                bucket: self,
                            });
                        }
                    }
                }
                Some(Err(e)) => {
                    eprintln!("error: {}", e);
                    break;
                }
                None => break,
            }
        }

        Ok(apps)
    }
}
