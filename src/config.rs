pub enum DbType {
    MySQL,
    PostgreSQL,
}
pub struct Config {
    config: Vec<Profile>,
}
pub struct Profile {
    db_type: DbType,
    hostname: String,
    username: String,
    password: String,
    port: u16,
}

impl Config {
    pub fn load(name: &str) -> Self {
        let mut config = config::Config::default();
        config
            .merge(config::File::with_name(name))
            .unwrap();
        Self {
            config: vec![]
        }
    }
}

// [stage]
//     file = "stage_config.json"
//     type = "mysql"
//     hostname = "{$file::deep.config.why.because.db.hostname}"
//     username = "{$file::deep.config.why.because.db.username}"
//     password = "{$file::deep.config.why.because.db.password}"
