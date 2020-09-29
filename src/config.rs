pub enum DbType {
    MySQL,
    PostgreSQL,
};
pub struct Config {
    profiles: Vec<Profile>,
}
pub struct Profile {
    db_type: DbType,
    hostname: String,
    username: String,
    password: String,
    port: u16,
};

// [stage]
//     file = "stage_config.json"
//     type = "mysql"
//     hostname = "{$file::deep.config.why.because.db.hostname}"
//     username = "{$file::deep.config.why.because.db.username}"
//     password = "{$file::deep.config.why.because.db.password}"
