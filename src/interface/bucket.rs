use crate::val::INSTALL_PATH;

use super::{app::App, manifest::Manifest};

pub async fn get_buckets() -> Result<Vec<Bucket>, anyhow::Error> {
    let mut buckets = Vec::new();

    let mut reader = tokio::fs::read_dir(INSTALL_PATH.clone().join("buckets")).await?;
    while let Ok(Some(entry)) = reader.next_entry().await {
        let bucket = Bucket::from_name(entry.file_name().to_str().unwrap());
        buckets.push(bucket);
    }

    Ok(buckets)
}

pub struct Bucket {
    pub name: String,
}

impl Bucket {
    pub fn from_name(name: &str) -> Self {
        Bucket {
            name: name.to_string(),
        }
    }

    fn path(&self) -> std::path::PathBuf {
        INSTALL_PATH.clone().join("buckets").join(&self.name)
    }

    pub async fn apps(&self) -> Result<Vec<App>, anyhow::Error> {
        let mut apps = Vec::new();
        let mut reader = tokio::fs::read_dir(self.path().join("bucket")).await?;
        while let Ok(Some(entry)) = reader.next_entry().await {
            let app = App::from_name(entry.path().file_stem().unwrap().to_str().unwrap());
            apps.push(app);
        }
        Ok(apps)
    }
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
