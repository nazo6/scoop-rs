use clap::{Args, Subcommand};

use super::CliResult;

pub mod install;
mod list;
pub mod search;
pub mod uninstall;
pub mod upgrade;

#[derive(Debug, Args)]
pub struct AppArgs {
    #[command(subcommand)]
    command: AppCommand,
}

#[derive(Subcommand, Debug)]
enum AppCommand {
    /// Install apps.
    #[command(visible_alias("i"))]
    Install(install::InstallArgs),

    /// Uninstall apps.
    #[command(visible_alias("un"))]
    Uninstall(uninstall::UninstallArgs),

    /// Upgrade apps.
    #[command(visible_alias("u"))]
    Upgrade(upgrade::UpgradeArgs),

    /// Upgrade apps.
    #[command(visible_alias("s"))]
    Search(search::SearchArgs),

    /// Show list of installed apps
    List,
}

pub async fn start(opts: AppArgs) -> CliResult {
    match opts.command {
        AppCommand::Install(args) => install::start(args).await,
        AppCommand::Uninstall(args) => uninstall::start(args).await,
        AppCommand::Upgrade(args) => upgrade::start(args).await,
        AppCommand::Search(args) => search::start(args).await,
        AppCommand::List => list::start().await,
    }
}
