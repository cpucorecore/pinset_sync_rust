use actix_web::{App, HttpServer};
use pinset_sync_rust::api::{hello, index, space_info, sync_review};
use pinset_sync_rust::settings::SETTINGS;
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("conf/log.yml", Default::default()).unwrap();

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hello)
            .service(space_info)
            .service(sync_review)
    })
    .workers(SETTINGS.api.worker)
    .client_request_timeout(Duration::from_secs(30))
    .keep_alive(Duration::from_secs(60))
    .bind((SETTINGS.api.host.clone(), SETTINGS.api.port))?
    .run()
    .await
}
