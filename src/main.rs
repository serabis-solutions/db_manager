use clap::clap_app;
use env_logger;
use log::debug;
use config;
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    static ref CONFIG: config::Config = {
        let mut config = config::Config::default();
        config.merge(config::File::with_name("dbmanager")).expect("error reading config");

        config
    };
}

fn main() {
    env_logger::init();

    let app = clap_app!(dbmanager =>
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
    );

    match app.get_matches().subcommand() {
        ("add", Some(sub_m)) => add(sub_m.value_of("name").unwrap()),
        ("deploy", Some(sub_m)) => deploy(
            sub_m.value_of("target").unwrap(),
            sub_m.values_of("name"),
        ),
        ("verify", Some(sub_m)) => verify(
            sub_m.value_of("target").unwrap(),
            sub_m.values_of("name"),
        ),
        ("revert", Some(sub_m)) => revert(
            sub_m.value_of("target").unwrap(),
            sub_m.values_of("name").unwrap(),
        ),
        ("list", Some(sub_m)) => match sub_m.subcommand() {
            ("available", Some(_)) => list_available(),
            ("deployed", Some(sub_m)) => list_deployed(sub_m.value_of("target").unwrap()),
            _ => panic!("no subcommand - shouldn't ever get here"),
        },
        _ => panic!("no subcommand - shouldn't ever get here"),
    }
}

fn add(name: &str) {
    debug!("add: \"{}\"", name);
}
fn deploy<'a>(target: &str, migrations: Option<impl Iterator<Item = &'a str> + std::fmt::Debug>) {
    debug!("deploy: \"{}\" [{:?}]", target, migrations);
}
fn verify<'a>(target: &str, migrations: Option<impl Iterator<Item = &'a str> + std::fmt::Debug>) {
    debug!("verify: \"{}\" [{:?}]", target, migrations);
}
fn revert<'a>(target: &str, migrations: impl Iterator<Item = &'a str> + std::fmt::Debug) {
    debug!("revert: \"{}\" [{:?}]", target, migrations);
}
fn list_available(){
    debug!("list_available {:?}", CONFIG.get_str("fpp"));
}
fn list_deployed(target: &str) {
    debug!("list_deployed: \"{}\"", target);
}
