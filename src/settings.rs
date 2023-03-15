use config::Config;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = Config::builder()
        .add_source(config::File::with_name("conf/config.toml"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();
}

pub fn get<'a, T: serde::Deserialize<'a>>(key: &str) -> T {
    CONFIG.get::<T>(key).unwrap()
}

#[test]
fn test_get() {
    let host: Vec<String> = get("network.host");
    println!("host:{:?}", host);
}
