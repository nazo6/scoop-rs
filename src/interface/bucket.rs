use serde::{Deserialize, Serialize};

use crate::val::INSTALL_PATH;

use super::{app::App, manifest::Manifest};

/// List of buckets
/// Usually, this represents all local buckets, but it is not guaranteed
pub struct BucketList {
    pub buckets: Vec<Bucket>,
}

impl BucketList {
    /// Load the list of buckets from the filesystem
    pub async fn load() -> Result<Self, anyhow::Error> {
        let mut buckets = Vec::new();
        let mut reader = tokio::fs::read_dir(INSTALL_PATH.clone().join("buckets")).await?;
        while let Ok(Some(entry)) = reader.next_entry().await {
            let bucket = Bucket::from_name(entry.file_name().to_str().unwrap());
            buckets.push(bucket);
        }
        Ok(BucketList { buckets })
    }
}

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn repository(&self) -> Result<git2::Repository, git2::Error> {
        git2::Repository::open(self.path())
    }

    /// Get list of apps in the bucket
    pub async fn apps(&self) -> Result<Vec<App>, anyhow::Error> {
        let mut apps = Vec::new();
        let mut reader = tokio::fs::read_dir(self.path().join("bucket")).await?;
        while let Ok(Some(entry)) = reader.next_entry().await {
            let app = App::from_name(entry.path().file_stem().unwrap().to_str().unwrap());
            apps.push(app);
        }
        Ok(apps)
    }

    /// Get list of apps in the bucket with their manifests
    pub async fn manifests(&self) -> Result<Vec<(App, Manifest)>, anyhow::Error> {
        let apps = self.apps().await?;
        let mut res = vec![];
        for app in apps {
            let manifest_path = self
                .path()
                .join("bucket")
                .join(format!("{}.json", app.name));
            res.push((app, Manifest::from_path(&manifest_path).await?));
        }

        Ok(res)
    }
}
