use log::*;
use pinset_sync_rust::http_client::do_post;

fn main() {
    env_logger::init();
    let rt = tokio::runtime::Runtime::new().unwrap();

    match rt.block_on(do_post()) {
        Ok(msg) => info!("Done {}", msg),
        Err(e) => error!("An error occurred: {}", e),
    };
}
