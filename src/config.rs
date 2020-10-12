mod profile;
use serde::Deserialize;
use std::collections::HashMap;
use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(with = "profile", rename(deserialize = "profile"))]
    profiles: HashMap<String, Profile>,
}
#[derive(Debug, Deserialize)]
pub struct Profile {
    name: String,
    #[serde(rename(deserialize = "type"))]
    db_type: DbType,
    hostname: Hostname,
    username: Username,
    password: Password,
    port: Option<u16>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DbType {
    MySQL,
    PostgreSQL,
}

//Todo custom deserialise on these that means they read the file
#[derive(Debug, Deserialize)]
struct Hostname(String);

#[derive(Debug, Deserialize)]
struct Username(String);

#[derive(Debug, Deserialize)]
struct Password(String);

impl Config {
    pub fn load(name: &str) -> Result<Self> {
        let mut config = config::Config::new();

        config.merge(config::File::with_name(name));

        config.try_into().map_err(::anyhow::Error::from)
    }
}
