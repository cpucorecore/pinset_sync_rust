use config::Config;
use log::*;
use pinset_sync_rust::http_client::do_post;
use std::collections::HashMap;

fn main() {
    log4rs::init_file("conf/log.yml", Default::default()).unwrap();
    let rt = tokio::runtime::Runtime::new().unwrap();

    let settings = Config::builder()
        .add_source(config::File::with_name("conf/config.toml"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    info!(
        "{:?}",
        settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    );

    match rt.block_on(do_post()) {
        Ok(msg) => info!("Done {}", msg),
        Err(e) => error!("An error occurred: {}", e),
    };
}
