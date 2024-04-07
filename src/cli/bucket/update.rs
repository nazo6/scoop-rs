use clap::Args;

use crate::cli::CliResult;

#[derive(Debug, Args)]
pub struct UpdateArgs {}

pub async fn start(opts: UpdateArgs) -> CliResult {
    println!("update");
    Ok(())
}
