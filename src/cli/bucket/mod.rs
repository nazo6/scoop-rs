use clap::{Args, Subcommand};

use super::CliResult;

mod list;
pub mod update;

#[derive(Debug, Args)]
pub struct BucketArgs {
    #[command(subcommand)]
    command: BucketCommand,
}

#[derive(Subcommand, Debug)]
pub enum BucketCommand {
    Add { name: String, url: String },
    Remove { name: String },
    Update(update::UpdateArgs),
    List {},
}

pub async fn start(opts: BucketArgs) -> CliResult {
    match opts.command {
        BucketCommand::Add { name, url } => {
            todo!()
        }
        BucketCommand::Remove { name } => {
            todo!()
        }
        BucketCommand::Update(args) => update::start(args).await,
        BucketCommand::List {} => list::start().await,
    }
}
