use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    fmt::{Display, Formatter},
    path::PathBuf,
    str::FromStr,
};

use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::{error::Result, val::INSTALL_PATH};

use super::{bucket::Bucket, installed_app::InstalledApp, manifest::Manifest};

/// Structure that represent one app in a bucket
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct BucketApp<'a> {
    pub name: String,
    pub metadata_path: PathBuf,
    pub bucket: &'a Bucket,
}

impl BucketApp<'_> {
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

    pub async fn manifest(&self) -> Result<Manifest> {
        Manifest::from_path(&self.metadata_path).await
    }
}

pub struct BucketsAppsRepository<'a> {
    pub inner: HashMap<Bucket, HashSet<BucketApp<'a>>>,
}

impl<'a> BucketsAppsRepository<'a> {
    pub async fn from_buckets(buckets: &'a [Bucket]) -> Result<Self> {
        let mut inner = HashMap::new();
        for bucket in buckets {
            let apps = bucket.apps().await?;
            inner.insert(bucket.clone(), apps);
        }
        Ok(BucketsAppsRepository { inner })
    }
}

/// Bucket app name and bucket name
/// This is used to represent `bucket/app` string
#[derive(DeserializeFromStr, SerializeDisplay, Debug, Clone)]
pub struct BucketAppName {
    pub bucket_name: Option<String>,
    pub name: String,
}

impl BucketAppName {
    pub fn get_bucket_app<'a>(
        &self,
        apps: &'a BucketsAppsRepository<'a>,
    ) -> Option<&'a BucketApp<'a>> {
        if let Some(bucket) = self.bucket_name.as_ref() {
            let bucket = Bucket::from_name(bucket);
            let app = apps
                .inner
                .get(&bucket)?
                .iter()
                .find(|a| a.name == self.name)?;
            Some(app)
        } else {
            let app = apps
                .inner
                .values()
                .find_map(|apps| apps.iter().find(|a| a.name == self.name))?;
            Some(app)
        }
    }
}

impl FromStr for BucketAppName {
    type Err = Infallible;
    fn from_str(s: &str) -> std::result::Result<BucketAppName, std::convert::Infallible> {
        if let Some((bucket_name, name)) = s.split_once('/') {
            Ok(BucketAppName {
                bucket_name: Some(bucket_name.to_string()),
                name: name.to_string(),
            })
        } else {
            Ok(BucketAppName {
                bucket_name: None,
                name: s.to_string(),
            })
        }
    }
}

impl Display for BucketAppName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(bucket_name) = &self.bucket_name {
            write!(f, "{}/{}", bucket_name, self.name)
        } else {
            write!(f, "{}", self.name)
        }
    }
}
