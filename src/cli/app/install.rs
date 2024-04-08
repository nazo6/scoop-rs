use std::collections::{HashMap, HashSet};

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

    let mut to_install = HashSet::new();
    for app_name in opts.apps {
        let app = apps
            .get(&app_name)
            .ok_or_else(|| format!("App `{}` not found in any bucket", app_name))?;
        to_install.insert(app.clone());
        deps::resolve_deps(app, &apps, &mut to_install)
            .await
            .map_err(|e| format!("Failed to resolve dependencies for `{}`: {}", app_name, e))?;
    }

    dbg!(to_install);

    Ok(())
}
