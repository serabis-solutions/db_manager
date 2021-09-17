use clap::{AppSettings, Clap, crate_version, crate_authors, crate_name, crate_description};

#[derive(Clap)]
#[clap(
    version = crate_version!(),
    author = crate_authors!(),
    name = crate_name!(),
    about = crate_description!(),
    setting = AppSettings::ColoredHelp,
    setting = AppSettings::SubcommandRequiredElseHelp
)]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clap)]
enum Command {
    Add(Add),
    Deploy(Deploy),
    Verify(Verify),
    Revert(Revert),
    List(List),
}

/// add a new migration
#[derive(Clap)]
struct Add {
    /// migration name
    #[clap(long)]
    name: String,
}

/// deploy all or a single migration
#[derive(Clap)]
struct Deploy {
    /// target to run against
    #[clap(long)]
    target: String,

    /// migration name(s) to deploy
    #[clap(long)]
    name: Option<Vec<String>>,
}

/// verify all or a single migration
#[derive(Clap)]
struct Verify {
    /// target to run against
    #[clap(long)]
    target: String,

    /// migration name(s) to verify
    #[clap(long)]
    name: Option<Vec<String>>,
}

/// revert a named migrations
#[derive(Clap)]
struct Revert {
    /// target to run against
    #[clap(long)]
    target: String,

    /// migration name(s) to revert
    #[clap(long)]
    name: Vec<String>,
}

/// list migrations
#[derive(Clap)]
#[clap(setting = AppSettings::SubcommandRequiredElseHelp)]
struct List {
    #[clap(subcommand)]
    what: ListWhat,
}

#[derive(Clap)]
enum ListWhat {
    Available(Available),
    Deployed(Deployed),
}

/// list available migrations
#[derive(Clap)]
struct Available {}

/// list deployed migrations
#[derive(Clap)]
struct Deployed {
    /// target to run against
    #[clap(long)]
    target: String,
}
