use tabled::{builder::Builder, settings::Style};

use crate::{cli::CliResult, interface::app::installed_apps};

pub async fn start() -> CliResult {
    println!("List of installed apps");

    let apps = installed_apps()
        .await
        .map_err(|e| format!("Failed to get apps: {}", e))?;

    let mut builder = Builder::default();
    builder.push_record(["Name", "Version", "Bucket"]);
    for app in apps {
        let current_version = app.current_version().await;
        let bucket = if let Ok(crr) = &current_version {
            if let Ok(i) = crr.install_info().await {
                i.bucket.name
            } else {
                "No current install found".to_string()
            }
        } else {
            "Failed to get install info".to_string()
        };
        let current_version = current_version
            .map(|v| v.version)
            .unwrap_or_else(|_| "Failed to get version".to_string());
        builder.push_record([app.name.as_str(), &current_version, &bucket]);
    }

    let table = builder.build().with(Style::rounded()).to_string();

    println!("{table}");
    Ok(())
}
