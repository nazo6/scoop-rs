use clap::Args;

use crate::cli::CliResult;

#[derive(Debug, Args)]
pub struct InstallArgs {
    pub name: String,
    pub no_hash_check: bool,
}

pub async fn start(opts: InstallArgs) -> CliResult {
    println!("Installing {}", opts.name);
    Ok(())
}
