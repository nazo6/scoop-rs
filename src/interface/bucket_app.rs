use std::path::PathBuf;

use crate::val::INSTALL_PATH;

use super::{bucket::Bucket, installed_app::InstalledApp, manifest::Manifest};

/// Structure that represent one app in a bucket
pub struct BucketApp<'a> {
    pub name: String,
    pub bucket: &'a Bucket,
}

impl BucketApp<'_> {
    pub fn path(&self) -> PathBuf {
        let mut path = INSTALL_PATH.clone();
        path.push("apps");
        path.push(&self.name);
        path
    }

    /// Check if the app is installed
    /// If the app is installed, return the InstalledApp
    pub async fn installed(&self) -> Option<InstalledApp> {
        let mut path = INSTALL_PATH.clone();
        path.push(&self.name);
        if path.exists() {
            Some(InstalledApp::from_name(&self.name))
        } else {
            None
        }
    }

    pub async fn manifest(&self) -> Result<Manifest, anyhow::Error> {
        let manifest_path = self
            .bucket
            .path()
            .join("bucket")
            .join(format!("{}.json", self.name));
        Manifest::from_path(&manifest_path).await
    }
}
