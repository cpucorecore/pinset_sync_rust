use crate::commands::get_disk_free_space;
use crate::db;
use crate::dependent_api::{ipfs_file_stat, ipfs_pin_ls, ipfs_repo_stat};
use crate::settings::SETTINGS;
use crate::types::{FileStat, IpfsRepoStat, PinSet, SpaceInfo};
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
    let mut space_pinned = 0_i64;

    for cid in cids {
        match db::pinset_get(cid.as_str()) {
            None => match ipfs_file_stat(&cid).await {
                Ok(api_resp) => match serde_json::from_str::<FileStat>(&api_resp) {
                    Ok(fs) => {
                        space_pinned += fs.cumulative_size;
                        db::pinset_set(cid.as_str(), &api_resp);
                    }
                    Err(err) => {
                        error!("parse FileStat err: {}", err);
                    }
                },
                Err(err) => {
                    error!("call api file stat err: {}", err);
                }
            },
            Some(db_resp) => {
                let fs = serde_json::from_str::<FileStat>(&db_resp).unwrap();
                space_pinned += fs.cumulative_size;
            }
        };
    }

    Ok(space_pinned)
}

#[allow(unused)]
async fn get_ipfs_file_stat2(cids: Vec<String>) -> Result<i64, ()> {
    let mut space_pinned = 0_i64;

    for cid in cids {
        match db::pinset_get2(cid.as_str()) {
            None => match ipfs_file_stat(&cid).await {
                Ok(api_resp) => match serde_json::from_str::<FileStat>(&api_resp) {
                    Ok(fs) => {
                        space_pinned += fs.cumulative_size;
                        db::pinset_set2(cid.as_str(), fs);
                    }
                    Err(err) => {
                        error!("parse FileStat err: {}", err);
                    }
                },
                Err(err) => {
                    error!("call api file stat err: {}", err);
                }
            },
            Some(fs) => {
                space_pinned += fs.cumulative_size;
            }
        };
    }

    Ok(space_pinned)
}

async fn get_pin_set() -> Result<Vec<String>, ()> {
    match ipfs_pin_ls().await {
        Ok(resp) => {
            debug!("pin ls response: {}", resp);

            let pinset: PinSet =
                serde_json::from_str(&resp).expect("json parse pin ls response failed"); // TODO: err handle
            let cids: Vec<String> = pinset.keys.into_keys().collect();
            Ok(cids)
        }
        Err(err) => {
            error!("ipfs pin ls err: {}", err);
            Err(())
        }
    }
}

async fn get_ipfs_repo_stat() -> Result<IpfsRepoStat, ()> {
    match ipfs_repo_stat().await {
        Ok(resp) => {
            debug!("ipfs repo stat: {}", resp);

            let stat: IpfsRepoStat =
                serde_json::from_str(&resp).expect("json parse repo stat response failed"); // TODO: err handle
            Ok(stat)
        }
        Err(err) => {
            error!("ipfs repo stat err: {}", err);
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

    let cids = get_pin_set().await.unwrap();

    let mut thread_handles = vec![];
    let worklists = split_worklists(cids);
    for worklist in worklists {
        thread_handles.push(spawn(get_ipfs_file_stat(worklist)));
    }

    for handle in thread_handles {
        space_pinned += handle.await.unwrap().unwrap()
    }

    let stat = get_ipfs_repo_stat().await.unwrap();

    serde_json::to_string(&SpaceInfo {
        space_pinned,
        space_used: stat.repo_size,
        space_ipfs_total: stat.storage_max,
        space_disk_free: get_disk_free_space(),
    })
    .unwrap()
}
