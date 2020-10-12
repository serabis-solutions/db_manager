use crate::config::Config;

#[test]
fn load() {
    let config = Config::load("test/config/config");
    panic!("{:#x?}", config);
}
