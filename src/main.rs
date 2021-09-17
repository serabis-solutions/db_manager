/* use clap::clap_app;
use env_logger;
use log::debug; */
use tracing::{debug, info, Level};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

mod args;

fn main() {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        // completes the builder.
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let number_of_yaks = 3;
    // this creates a new event, outside of any spans.
    info!(number_of_yaks, "preparing to shave yaks");

    let number_shaved = 3;
    debug!(
        all_yaks_shaved = number_shaved == number_of_yaks,
        "yak shaving completed."
    );

    /* env_logger::init();

    let matches = clap_app!(db_manager =>
        (version: "1.0")
        (author: "Mark Ellis <m@rkellis.com>")
        (about: "Manages database migrations")
        (@setting SubcommandRequiredElseHelp)
        (@subcommand add =>
            (about: "add a new migration")
            (@arg name: <name> "migration name")
        )
        (@subcommand deploy =>
            (about: "deploy all or a single migration")
            (@arg target: <target> "target to run against")
            (@arg name: [name]... "migration name(s)")
        )
        (@subcommand verify =>
            (about: "verify all or a single migration")
            (@arg target: <target> "target to run against")
            (@arg name: [name]... "migration name(s)")
        )
        (@subcommand revert =>
            (about: "revert a single migration")
            (@arg target: <target> "target to run against")
            (@arg name: <name>... "migration name(s)")
        )
        (@subcommand list =>
            (@setting SubcommandRequiredElseHelp)
            (about: "list migrations")
            (@subcommand available =>
                (about: "list available migrations")
            )
            (@subcommand deployed =>
                (about: "list deployed migrations")
                (@arg target: <target> "target to run against")
            )
        )
    )
    .get_matches();

    //read config
    let config = "I am the config now";

    match matches.subcommand() {
        ("add", Some(sub_m)) => add(&config, sub_m.value_of("name").unwrap()),
        ("deploy", Some(sub_m)) => deploy(
            &config,
            sub_m.value_of("target").unwrap(),
            sub_m.values_of("name"),
        ),
        ("verify", Some(sub_m)) => verify(
            &config,
            sub_m.value_of("target").unwrap(),
            sub_m.values_of("name"),
        ),
        ("revert", Some(sub_m)) => revert(
            &config,
            sub_m.value_of("target").unwrap(),
            sub_m.values_of("name").unwrap(),
        ),
        ("list", Some(sub_m)) => match sub_m.subcommand() {
            ("available", Some(_)) => list_available(&config),
            ("deployed", Some(sub_m)) => list_deployed(&config, sub_m.value_of("target").unwrap()),
            _ => panic!("no subcommand - shouldn't ever get here"),
        },
        _ => panic!("no subcommand - shouldn't ever get here"),
    } */
}

fn add(config: &str, name: &str) {
    debug!("add: \"{}\"", name);
}
fn deploy<'a>(
    config: &str,
    target: &str,
    migrations: Option<impl Iterator<Item = &'a str> + std::fmt::Debug>,
) {
    debug!("deploy: \"{}\" [{:?}]", target, migrations);
}
fn verify<'a>(
    config: &str,
    target: &str,
    migrations: Option<impl Iterator<Item = &'a str> + std::fmt::Debug>,
) {
    debug!("verify: \"{}\" [{:?}]", target, migrations);
}
fn revert<'a>(
    config: &str,
    target: &str,
    migrations: impl Iterator<Item = &'a str> + std::fmt::Debug,
) {
    debug!("revert: \"{}\" [{:?}]", target, migrations);
}
fn list_available(config: &str) {
    debug!("list_available");
}
fn list_deployed(config: &str, target: &str) {
    debug!("list_deployed: \"{}\"", target);
}
