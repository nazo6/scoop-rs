use std::collections::HashMap;

use clap::Args;
use interface::bucket::get_buckets;

use crate::cli::CliResult;

mod deps;

#[derive(Debug, Args)]
pub struct InstallArgs {
    #[clap(required = true)]
    pub apps: Vec<String>,
    #[clap(long, default_value_t = false)]
    pub no_hash_check: bool,
}

pub async fn start(opts: InstallArgs) -> CliResult {
    let buckets = get_buckets()
        .await
        .map_err(|e| format!("Failed to get buckets: {}", e))?;
    let mut apps = HashMap::new();
    for bucket in &buckets {
        for app in bucket
            .apps()
            .await
            .map_err(|e| format!("Failed to get apps of bucket: {} ({})", bucket.name, e))?
        {
            apps.insert(app.name.clone(), app);
        }
    }

    let mut to_install = Vec::new();

    for app_name in opts.apps {
        let app = apps
            .get(&app_name)
            .ok_or_else(|| format!("App `{}` not found in any bucket.", app_name,))?;
        let manifest = app.manifest()
            .await
            .map_err(|e| format!("Failed to get manifest of app `{}` from bucket `{}`: {}\nThis may be caused by invalid bucket.",app.name, app.bucket.name, e))?;
        to_install.push((app, manifest));
    }

    Ok(())
}
