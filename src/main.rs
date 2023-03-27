extern crate core;

use actix_web::{web, App, HttpServer};
use pinset_sync_rust::api::{
    gc, get_state, space_info, start_ipfs, start_ipfs_cluster, sync, sync_review,
};
use pinset_sync_rust::db;
use pinset_sync_rust::ipfs_cluster_proxy as cluster_api;
use pinset_sync_rust::settings::S;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::process::exit;
use std::thread;
use tokio::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup().await;

    let mut signals = Signals::new(&[SIGINT])?;
    let j = thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
            db::flush();
            exit(0);
        }
    });

    let tx_id = web::Data::new(Mutex::new(0));

    let result = HttpServer::new(move || {
        App::new()
            .app_data(tx_id.clone())
            .service(get_state)
            .service(sync_review)
            .service(sync)
            .service(space_info)
            .service(gc)
            .service(start_ipfs)
            .service(start_ipfs_cluster)
    })
    .workers(S.api.worker)
    .bind((S.api.host.clone(), S.api.port))?
    .run()
    .await;

    j.join().unwrap();

    result
}

async fn setup() {
    log4rs::init_file("conf/log.yml", Default::default()).unwrap();
    if let None = db::get_cluster_id() {
        if let Some(id) = cluster_api::id().await {
            db::save_cluster_id(id.id)
        } else {
            panic!("setup: call api(cluster id) failed"); // TODO: not panic for deploy ipfs?
        }
    }
}
