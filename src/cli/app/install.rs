use clap::Args;

use crate::cli::CliResult;

#[derive(Debug, Args)]
pub struct InstallArgs {
    #[clap(required = true)]
    pub apps: Vec<String>,
    #[clap(long)]
    pub no_hash_check: bool,
}

pub async fn start(opts: InstallArgs) -> CliResult {
    Ok(())
}
