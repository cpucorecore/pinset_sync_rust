use crate::http_client::ipfs_repo_stat;
use crate::types::SpaceInfo;
use actix_web::{get, web, Responder};

#[get("/ipfs_repo_stat")]
pub async fn index() -> impl Responder {
    match ipfs_repo_stat().await {
        Ok(msg) => format!("do post response:{}", msg),
        Err(err) => format!("do post err:{}", err),
    }
}

#[get("/restful/{name}")]
pub async fn hello(name: web::Path<String>) -> impl Responder {
    format!("hello {}!", &name)
}

#[get("/space_info")]
pub async fn space_info() -> impl Responder {
    let space_pinned = 0;
    let space_used = 0;
    let space_ipfs_total = 0;
    let space_disk_free = 0;

    serde_json::to_string(&SpaceInfo {
        space_pinned,
        space_used,
        space_ipfs_total,
        space_disk_free,
    })
    .unwrap()
}
