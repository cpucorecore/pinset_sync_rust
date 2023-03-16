use actix_web::{get, web, App, HttpServer, Responder};
use pinset_sync_rust::http_client::ipfs_repo_stat;
use std::time::Duration;

#[get("/ipfs_repo_stat")]
async fn index() -> impl Responder {
    match ipfs_repo_stat().await {
        Ok(msg) => format!("do post response:{}", msg),
        Err(err) => format!("do post err:{}", err),
    }
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("conf/log.yml", Default::default()).unwrap();

    HttpServer::new(|| App::new().service(index).service(hello))
        .workers(2)
        .client_request_timeout(Duration::from_secs(30))
        .keep_alive(Duration::from_secs(60))
        .bind(("127.0.0.1", 8888))?
        .run()
        .await
}
