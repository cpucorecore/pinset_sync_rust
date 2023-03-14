// use pinset_sync_rust::commands::{
//     export_cluster_state, ipfs_pin_ls, start_cluster, start_ipfs, stop_cluster, stop_ipfs,
// };
use std::fmt::Error;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;

use futures::prelude::*;
use log::*;
use reqwest::Response;

async fn app() -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let x = client
        .post("http://192.168.0.85:5001/api/v0/repo/stat?size-only=false&human=false")
        .send()
        .await?;

    Ok(x.text().await.unwrap())
}

fn main() {
    env_logger::init();
    let rt = tokio::runtime::Runtime::new().unwrap();

    match rt.block_on(app()) {
        Ok(msg) => println!("Done {}", msg),
        Err(e) => error!("An error ocurred: {}", e),
    };

    // let blocking_task = tokio::task::spawn_blocking(|| {
    //     // This is running on a blocking thread.
    //     // Blocking here is ok.
    //     let client = reqwest::Client::new();
    //     client
    //         .post("http://192.168.0.85:5001/api/v0/repo/stat?size-only=false&human=false")
    //         .send()
    // });
    // blocking_task.await.unwrap();
    // do_post();
    // stop_cluster();
    // export_cluster_state();
    // start_cluster();
    //
    // stop_ipfs();
    // ipfs_pin_ls();
    // start_ipfs();

    sleep(Duration::new(1, 0))
}
