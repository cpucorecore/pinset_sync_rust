use log::*;
use pinset_sync_rust::http_client::do_post;

fn main() {
    log4rs::init_file("conf/log.yml", Default::default()).unwrap();
    let rt = tokio::runtime::Runtime::new().unwrap();

    match rt.block_on(do_post()) {
        Ok(msg) => info!("Done {}", msg),
        Err(e) => error!("An error occurred: {}", e),
    };
}
