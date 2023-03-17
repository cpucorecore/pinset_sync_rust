use crate::dependent_api::{ipfs_file_stat, ipfs_pin_ls, ipfs_repo_stat};
use crate::settings::SETTINGS;
use crate::types::{FileStat, PinSet, SpaceInfo};
use actix_web::rt::spawn;
use actix_web::{get, web, Responder};
use log::{debug, error};

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

async fn get_ipfs_file_stat(cids: Vec<String>) -> Result<i64, ()> {
    let mut space_pinned1 = 0_i64;

    for cid in cids {
        match ipfs_file_stat(&cid).await {
            Ok(resp) => {
                let fs: FileStat = serde_json::from_str(&resp).expect("");
                println!("{:?}", fs);
                space_pinned1 += fs.cumulative_size;
            }
            Err(err) => {
                error!("err: {}", err);
            }
        }
    }

    Ok(space_pinned1)
}

async fn get_pin_set() -> Result<Vec<String>, ()> {
    match ipfs_pin_ls().await {
        Ok(pin_ls_resp) => {
            debug!("pin ls response: {}", pin_ls_resp);

            let pinset: PinSet =
                serde_json::from_str(&pin_ls_resp).expect("json parse pin ls response failed");
            let cids: Vec<String> = pinset.keys.into_keys().collect();
            Ok(cids)
        }
        Err(err) => {
            error!("ipfs pin ls err: {}", err);
            Err(())
        }
    }
}

fn split_worklists(mut cids: Vec<String>) -> Vec<Vec<String>> {
    let mut worklists = vec![];

    let batch_size = cids.len() / SETTINGS.dependent_api.worker;
    let _ = (0..SETTINGS.dependent_api.worker - 1).map(|_| {
        let batch: Vec<String> = cids.drain(0..batch_size).collect();
        worklists.push(batch);
    });

    let last_batch: Vec<String> = cids.drain(0..cids.len()).collect();
    worklists.push(last_batch);

    worklists
}

#[get("/space_info")]
pub async fn space_info() -> impl Responder {
    let mut space_pinned = 0_i64;
    let space_used = 0;
    let space_ipfs_total = 0;
    let space_disk_free = 0;

    let cids = get_pin_set().await.unwrap();

    let mut thread_handles = vec![];
    let worklists = split_worklists(cids);
    for worklist in worklists {
        thread_handles.push(spawn(get_ipfs_file_stat(worklist)));
    }

    for handle in thread_handles {
        space_pinned += handle.await.unwrap().unwrap()
    }

    serde_json::to_string(&SpaceInfo {
        space_pinned,
        space_used,
        space_ipfs_total,
        space_disk_free,
    })
    .unwrap()
}
