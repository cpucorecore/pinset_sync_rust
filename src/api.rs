use crate::http_client::{file_stat, ipfs_pin_ls, ipfs_repo_stat};
use crate::types::{FileStat, PinSet, SpaceInfo};
use actix_web::{get, web, Responder};
use log::error;

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
    let mut space_pinned = 0_i64;
    let space_used = 0;
    let space_ipfs_total = 0;
    let space_disk_free = 0;

    // TODO: extract a private function below
    if let Ok(pin_ls_resp) = ipfs_pin_ls().await {
        println!("xxx: {}", pin_ls_resp);
        let pinset: PinSet = serde_json::from_str(&pin_ls_resp).expect("");
        for (pin, _) in pinset.keys {
            //TODO: if need call api, use multi thread
            // TODO: query db first for reduce api(file_stat) calls
            println!("{:?}", pin);
            match file_stat(&pin).await {
                Ok(file_stat_resp) => {
                    let fs: FileStat = serde_json::from_str(&file_stat_resp).expect("");
                    println!("{:?}", fs);
                    space_pinned += fs.cumulative_size;
                    //TODO: save to db for deduplicated request
                }
                Err(err) => {
                    error!("err: {}", err);
                }
            }
        }
    }

    serde_json::to_string(&SpaceInfo {
        space_pinned,
        space_used,
        space_ipfs_total,
        space_disk_free,
    })
    .unwrap()
}
