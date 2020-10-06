use std::collections::HashMap;

pub struct Config {
    config: HashMap<String, Profile>,
}
pub struct Profile {
    name: String,
    db_type: DbType,
    hostname: Hostname,
    username: Username,
    password: Password,
    port: u16,
}
pub enum DbType {
    MySQL,
    PostgreSQL,
}

//Todo custom deserialise on these that means they read the file
struct Hostname(String);
struct Username(String);
struct Password(String);

impl Config {
    pub fn load(name: &str) -> Self {
        let mut config = config::Config::default();
        config
            .merge(config::File::with_name(name))
            .unwrap();

        //call tryinto<Config>
        Self {
            config: HashMap::new()
        }
    }
}

// [stage]
//     file = "stage_config.json"
//     type = "mysql"
//     hostname = "{$file::deep.config.why.because.db.hostname}"
//     username = "{$file::deep.config.why.because.db.username}"
//     password = "{$file::deep.config.why.because.db.password}"
