use crate::{cli::CliResult, interface::app::installed_apps};

pub async fn start() -> CliResult {
    println!("List of installed apps");
    let apps = installed_apps()
        .await
        .map_err(|e| format!("Failed to get apps: {}", e))?;
    let str = apps
        .iter()
        .map(|app| app.name.clone())
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", str);
    Ok(())
}
