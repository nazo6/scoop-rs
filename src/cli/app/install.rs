use anyhow::Context;
use clap::Args;
use interface::{
    bucket::get_buckets,
    bucket_app::{BucketAppName, BucketsAppsRepository},
};

use crate::cli::CliResult;

mod download;
mod env;
mod installer;
mod link;
mod persist;
mod resolve;
mod run_script;
mod shortcut;

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

    // TODO: Error handling
    download::download(&install_apps).await;

    for (app, manifest) in install_apps {
        let arch_m = manifest.architecture_current();

        if let Some(pre_install) = &arch_m.pre_install {
            run_script::run_script(pre_install)
                .await
                .context("Failed to run pre-install script")?;
        }

        installer::extract(app, &manifest).await?;
        installer::run_installer(app, &manifest).await?;

        link::link_to_current(app, &manifest.version).await?;

        shortcut::create_shims(app, &manifest).await?;
        shortcut::create_startmenu_shortcuts(app, &manifest).await?;

        installer::install_psmodule(app, &manifest).await?;

        env::path(app, &manifest).await?;
        env::set_env(app, &manifest).await?;

        persist::persist(app, &manifest).await?;

        if let Some(post_install) = &arch_m.post_install {
            run_script::run_script(post_install)
                .await
                .context("Failed to run post-install script")?;
        }

        installer::create_info(app, &manifest).await?;

        println!("Installed {}", app.name);
    }

    Ok(())
}
