mod cli;
mod interface;
mod val;

#[tokio::main]
async fn main() {
    let scoop = interface::installed_app::InstalledApp::from_name("scoop");

    #[cfg(debug_assertions)]
    {
        println!("Debug mode is enabled. Skipping the check for scoop-rs installation.\n");
        cli::start().await;
        return;
    }

    if scoop.is_installed().await {
        let versions = scoop
            .versions()
            .await
            .expect("Failed to get scoop-rs versions. Maybe the installation is corrupted");
    } else if dialoguer::Confirm::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("scoop-rs is not installed. Do you want to install it?")
        .default(true)
        .wait_for_newline(true)
        .interact()
        .unwrap()
    {
        println!("Looks like you want to continue");
        cli::start().await;
    } else {
        println!("Aborted.");
    }
}
