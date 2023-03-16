use actix_web::{App, HttpServer};
use pinset_sync_rust::api::{hello, index, space_info};
use pinset_sync_rust::settings;
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("conf/log.yml", Default::default()).unwrap();

    let rpc_host: String = settings::get("rpc.host");
    let rpc_port: u16 = settings::get("rpc.port");
    let rpc_worker: usize = settings::get("rpc.worker");

    HttpServer::new(|| App::new().service(index).service(hello).service(space_info))
        .workers(rpc_worker)
        .client_request_timeout(Duration::from_secs(30))
        .keep_alive(Duration::from_secs(60))
        .bind((rpc_host, rpc_port))?
        .run()
        .await
}
