use clap::{Parser, Subcommand};

mod app;
mod bucket;

type CliResult = Result<(), String>;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Install apps. This is alias of `app install`
    #[command(visible_alias("i"))]
    Install(app::install::InstallArgs),

    /// Uninstall apps. This is alias of `app uninstall`
    #[command(visible_alias("un"))]
    Uninstall(app::uninstall::UninstallArgs),

    /// Upgrade apps. This is alias of `app upgrade`
    #[command(visible_alias("up"))]
    Upgrade(app::upgrade::UpgradeArgs),

    /// Update buckets. This is alias of `bucket update`
    #[command(visible_alias("u"))]
    Update(bucket::update::UpdateArgs),

    /// Search apps. This is alias of `app search`
    Search(app::search::SearchArgs),

    /// Manage apps
    App(app::AppArgs),

    /// Manage buckets
    Bucket(bucket::BucketArgs),
}

pub async fn start() {
    let cli = Cli::parse();

    let res: CliResult = match cli.command {
        Command::Install(args) => app::install::start(args).await,
        Command::Uninstall(args) => app::uninstall::start(args).await,
        Command::Upgrade(args) => app::upgrade::start(args).await,
        Command::Update(args) => bucket::update::start(args).await,
        Command::Search(args) => app::search::start(args).await,
        Command::App(args) => app::start(args).await,
        Command::Bucket(args) => bucket::start(args).await,
    };

    if let Err(msg) = res {
        eprintln!("{}{}", console::style("Error: ").red(), msg);
    }
}
