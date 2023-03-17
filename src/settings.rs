use config::{Config, ConfigError, Environment, File};
use lazy_static::lazy_static;
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Api {
    pub host: String,
    pub port: u16,
    pub worker: usize,
}

#[derive(Debug, Deserialize)]
pub struct DependentApi {
    pub host: String,
    pub ipfs_port: u16,
    pub ipfs_cluster_port: u16,
    pub worker: usize,
}

#[derive(Debug, Deserialize)]
pub struct Db {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub api: Api,
    pub dependent_api: DependentApi,
    pub db: Db,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or("development".into());

        let config = Config::builder()
            .add_source(File::with_name("conf/default"))
            .add_source(File::with_name(&format!("conf/{}", run_mode)).required(false))
            .add_source(File::with_name("conf/local").required(false))
            .add_source(Environment::with_prefix("app"))
            .build()?;

        config.try_deserialize()
    }
}

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().unwrap();
}

#[test]
fn test_settings() {
    println!("{:?}", SETTINGS.dependent_api);
}
