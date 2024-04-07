use clap::Args;

use crate::cli::CliResult;

#[derive(Debug, Args)]
pub struct UpgradeArgs {
    /// The app to upgrade
    /// If no package is specified, all packages are upgraded
    name: Option<Vec<String>>,
}

pub async fn start(opts: UpgradeArgs) -> CliResult {
    println!("upgrade");
    Ok(())
}
