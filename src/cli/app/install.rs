use anyhow::Context;
use clap::Args;
use interface::{
    bucket::get_buckets,
    bucket_app::{BucketAppName, BucketsAppsRepository},
};

use crate::cli::CliResult;

mod download;
mod resolve;

#[derive(Debug, Args)]
pub struct InstallArgs {
    #[clap(required = true)]
    pub apps: Vec<BucketAppName>,
    #[clap(long, default_value_t = false)]
    pub no_hash_check: bool,
}

pub async fn start(opts: InstallArgs) -> CliResult {
    start_inner(opts).await.map_err(|e| e.to_string())
}

pub async fn start_inner(opts: InstallArgs) -> anyhow::Result<()> {
    let buckets = get_buckets().await.context("Failed to get buckets")?;
    let apps = BucketsAppsRepository::from_buckets(&buckets)
        .await
        .context("Failed to get apps from buckets")?;

    let mut install_apps = Vec::new();
    for app_name in opts.apps {
        let app = app_name
            .get_bucket_app(&apps)
            .context("App not found in any bucket")?;
        let to_install = resolve::resolve(app, &apps)
            .await
            .context("Failed to resolve dependencies")?;
        install_apps.extend(to_install);
    }

    download::download(&install_apps).await;

    Ok(())
}
