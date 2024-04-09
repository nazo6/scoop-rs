use clap::Args;
use interface::bucket::get_buckets;
use tabled::settings::Style;

use crate::cli::CliResult;

#[derive(Debug, Args)]
pub struct SearchArgs {
    pub query: String,
}

pub async fn start(opts: SearchArgs) -> CliResult {
    let buckets = get_buckets()
        .await
        .map_err(|e| format!("Failed to get buckets: {}", e))?;
    let mut apps = vec![];
    for bucket in &buckets {
        let bucket_apps = bucket
            .apps()
            .await
            .map_err(|e| format!("Failed to get apps: {}", e))?;
        apps.extend(bucket_apps);
    }

    let mut builder = tabled::builder::Builder::default();
    builder.push_record(["Name", "Bucket", "Version"]);

    for app in apps {
        if app.name.contains(&opts.query) {
            let version = app
                .manifest()
                .await
                .map(|m| m.version)
                .unwrap_or_else(|_| "Failed to get version".to_string());
            builder.push_record([app.name.as_str(), app.bucket.name.as_str(), &version]);
        }
    }

    let table = builder.build().with(Style::rounded()).to_string();

    println!("Search results for '{}'", opts.query);
    println!("{}", table);

    Ok(())
}
