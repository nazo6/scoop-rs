use clap::Args;

use crate::cli::CliResult;

#[derive(Debug, Args)]
pub struct UninstallArgs {
    pub name: String,
}

pub async fn start(opts: UninstallArgs) -> CliResult {
    println!("Uninstalling {}", opts.name);
    Ok(())
}
