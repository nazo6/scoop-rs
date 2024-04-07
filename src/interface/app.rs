use std::path::PathBuf;

use anyhow::Context;

use crate::val::INSTALL_PATH;

pub async fn installed_apps() -> Result<Vec<App>, anyhow::Error> {
    let mut apps = Vec::new();
    let mut readdir = tokio::fs::read_dir(INSTALL_PATH.clone().join("apps"))
        .await
        .context("Failed to read apps directory")?;
    while let Ok(Some(entry)) = readdir.next_entry().await {
        if let Some(name) = entry.file_name().to_str() {
            apps.push(App::from_name(name));
        }
    }
    Ok(apps)
}

/// Structure that represent one installed app
pub struct App {
    pub name: String,
}

impl App {
    pub fn from_name(name: &str) -> Self {
        App {
            name: name.to_string(),
        }
    }

    pub async fn is_installed(&self) -> bool {
        let mut path = INSTALL_PATH.clone();
        path.push(&self.name);
        path.exists()
    }

    pub async fn versions(&self) -> Result<Vec<AppVersion>, anyhow::Error> {
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
}

pub struct AppVersion<'a> {
    app: &'a App,
    pub version: String,
}

impl AppVersion<'_> {
    pub fn dir(&self) -> PathBuf {
        let mut path = INSTALL_PATH.clone();
        path.push(&self.app.name);
        path.push(&self.version);
        path
    }
}
