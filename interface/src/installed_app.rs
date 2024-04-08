use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::val::INSTALL_PATH;
use crate::Context as _;

use super::{bucket::Bucket, manifest::Manifest};

pub async fn installed_apps() -> Result<Vec<InstalledApp>> {
    let mut apps = Vec::new();
    let mut readdir = tokio::fs::read_dir(INSTALL_PATH.clone().join("apps")).await?;
    while let Ok(Some(entry)) = readdir.next_entry().await {
        if let Some(name) = entry.file_name().to_str() {
            apps.push(InstalledApp::from_name(name));
        }
    }
    Ok(apps)
}

/// Structure that represent one installed app
pub struct InstalledApp {
    pub name: String,
}

impl InstalledApp {
    pub fn from_name(name: &str) -> Self {
        InstalledApp {
            name: name.to_string(),
        }
    }

    pub fn path(&self) -> PathBuf {
        let mut path = INSTALL_PATH.clone();
        path.push("apps");
        path.push(&self.name);
        path
    }

    pub async fn is_installed(&self) -> bool {
        let mut path = INSTALL_PATH.clone();
        path.push(&self.name);
        path.exists()
    }

    pub async fn versions(&self) -> Result<Vec<AppVersion>> {
        let mut path = INSTALL_PATH.clone();
        path.push(&self.name);
        let mut versions = Vec::new();
        let mut readdir = tokio::fs::read_dir(path).await?;
        while let Ok(Some(entry)) = readdir.next_entry().await {
            if let Some(version) = entry.file_name().to_str() {
                // `current` is a symlink to the current version of the app
                if version == "current" {
                    continue;
                }
                versions.push(AppVersion {
                    app: self,
                    version: version.to_string(),
                });
            }
        }
        Ok(versions)
    }

    pub async fn current_version(&self) -> Result<AppVersion> {
        let path = self.path().join("current");
        if !path.exists() {
            return Err(crate::error::Error::InvalidState(
                "No current version".to_string(),
            ));
        }
        let version = tokio::fs::read_link(&path)
            .await?
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        Ok(AppVersion { app: self, version })
    }
}

#[derive(Serialize, Deserialize)]
pub struct AppInstallInfo {
    pub bucket: Bucket,
    pub architecture: String,
}

/// Structure that represent one installed app version
pub struct AppVersion<'a> {
    app: &'a InstalledApp,
    pub version: String,
}

impl AppVersion<'_> {
    pub fn path(&self) -> PathBuf {
        self.app.path().join(&self.version)
    }
    pub async fn install_info(&self) -> Result<AppInstallInfo> {
        let mut path = self.path();
        path.push("install.json");

        let content = tokio::fs::read_to_string(&path).await?;
        serde_json::from_str(&content).map_err(|e| Error::JsonParse("install.json", e))
    }
    /// Get the manifest of this install
    pub async fn manifest(&self) -> Result<Manifest> {
        Manifest::from_path(&self.path().join("manifest.json"))
            .await
            .with_context(|| format!("Failed to get manifest of {}", self.version))
    }
}
