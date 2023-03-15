use log::*;
use pinset_sync_rust::http_client::do_post;
use pinset_sync_rust::settings;

fn main() {
    log4rs::init_file("conf/log.yml", Default::default()).unwrap();
    let rt = tokio::runtime::Runtime::new().unwrap();

    let s: Vec<String> = settings::get("network.host");
    info!("{:?}", s);

    match rt.block_on(do_post()) {
        Ok(msg) => info!("Done {}", msg),
        Err(e) => error!("An error occurred: {}", e),
    };
}
