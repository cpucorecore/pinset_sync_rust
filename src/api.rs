use crate::commands::{
    cmd_ipfs_pin_ls, export_cluster_state, get_disk_free_space, ipfs_gc, start_cluster, start_ipfs,
    stop_cluster, stop_ipfs,
};
use crate::db;
use crate::dependent_api::{
    cluster_id, cluster_pin_ls, ipfs_file_stat, ipfs_pin_add, ipfs_pin_ls, ipfs_pin_rm,
    ipfs_repo_stat,
};
use crate::settings::SETTINGS;
use crate::types::{FileStat, IpfsRepoStat, SpaceInfo, SyncReview};
use actix_web::rt::spawn;
use actix_web::{get, web, Responder};
use log::{debug, error, info};

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
    serde_json::to_string(&get_space_info().await).unwrap()
}

pub async fn get_space_info() -> Option<SpaceInfo> {
    let mut space_pinned = 0_i64;

    let cids = ipfs_pin_ls().await.unwrap();

    let mut thread_handles = vec![];
    let worklists = split_worklists(cids);
    for worklist in worklists {
        thread_handles.push(spawn(get_ipfs_file_stat(worklist)));
    }

    for handle in thread_handles {
        space_pinned += handle.await.unwrap().unwrap()
    }

    let stat = get_ipfs_repo_stat().await.unwrap();

    Some(SpaceInfo {
        space_pinned,
        space_used: stat.repo_size,
        space_ipfs_total: stat.storage_max,
        space_disk_free: get_disk_free_space(),
    })
}

#[get("/gc_review")]
pub async fn gc_review() -> impl Responder {
    let si = get_space_info().await.unwrap();
    serde_json::to_string(&si).unwrap()
}

#[get("/gc")]
pub async fn gc() -> impl Responder {
    let id = "12D3KooWJ7b5LSbZJmRvrgGQVoSyVM6bTQdjtSc6cBpLWoZTQKXH".to_string(); // TODO: get from db
                                                                                 // TODO: 1. global status for query; 2. session; 3. gc lock
    if let None = stop_cluster() {
        return "stop cluster failed";
    }
    info!("cluster stopped");

    if let None = stop_ipfs() {
        // TODO: restart cluster
        return "stop ipfs failed";
    }
    info!("ipfs stopped");

    match export_cluster_state() {
        None => {
            // TODO: restart cluster and ipfs
            error!("cluster state export failed");
            return "cluster state export failed";
        }
        Some(cluster_pinset) => {
            info!("cluster state export finish");
            match cmd_ipfs_pin_ls() {
                None => {
                    // TODO: restart cluster and ipfs
                    error!("ipfs pin ls failed");
                    return "ipfs pin ls failed";
                }
                Some(ipfs_pinset) => {
                    info!("ipfs pin ls finish");
                    match ipfs_gc() {
                        None => {
                            // TODO: restart cluster and ipfs
                            error!("ipfs gc failed");
                            return "ipfs gc failed";
                        }
                        Some(_) => {
                            info!("ipfs gc finish");
                            match start_ipfs() {
                                None => {
                                    // TODO: restart cluster and ipfs
                                    error!("ipfs start failed");
                                    return "ipfs start failed";
                                }
                                Some(ipfs_pid) => {
                                    info!("ipfs started, pid: {}", ipfs_pid);

                                    let mut pinset_should_pin = vec![];
                                    let mut review = SyncReview {
                                        pins_to_add: vec![],
                                        pins_to_rm: vec![],
                                    };

                                    for cluster_pin in cluster_pinset {
                                        if cluster_pin.allocations.contains(&id) {
                                            pinset_should_pin.push(cluster_pin.cid)
                                        }
                                    }

                                    for cid in &pinset_should_pin {
                                        if !ipfs_pinset.contains(cid) {
                                            review.pins_to_add.push(cid.clone())
                                        }
                                    }

                                    for cid in &ipfs_pinset {
                                        if !pinset_should_pin.contains(&cid) {
                                            review.pins_to_rm.push(cid.clone())
                                        }
                                    }

                                    debug!("do_sync begin"); //TODO wait ipfs api setup finish
                                    do_sync(&review).await;
                                    debug!("do_sync finish");

                                    match start_cluster() {
                                        None => {
                                            error!("start cluster failed");
                                        }
                                        Some(cluster_pid) => {
                                            info!("cluster started, pid: {}", cluster_pid);
                                            return "ok";
                                        }
                                    }
                                }
                            }
                            ""
                        }
                    }
                }
            }
        }
    }
}

#[get("/sync_review")]
pub async fn sync_review() -> impl Responder {
    match get_sync_review().await {
        Some(review) => serde_json::to_string(&review).unwrap(),
        None => "get_sync_review failed".to_string(),
    }
}

#[get("/sync")]
pub async fn sync() -> impl Responder {
    // TODO: 1. sync lock; 2. sync status;
    match get_sync_review().await {
        Some(review) => {
            do_sync(&review).await;
            "ok"
        }
        None => "get_sync_review failed",
    }
}

async fn do_sync(review: &SyncReview) {
    // TODO: multiple thread do it
    for cid in &review.pins_to_add {
        match ipfs_pin_add(cid).await {
            Ok(resp) => {
                debug!("ipfs pin add resp:{}", resp);
            }
            Err(err) => {
                error!("ipfs pin add err: {}", err);
            }
        }
    }

    for cid in &review.pins_to_rm {
        match ipfs_pin_rm(cid).await {
            Ok(resp) => {
                debug!("ipfs pin rm resp:{}", resp);
            }
            Err(err) => {
                error!("ipfs pin rm err: {}", err);
            }
        }
    }
}

async fn get_sync_review() -> Option<SyncReview> {
    let id;
    match cluster_id().await {
        Some(r) => id = r,
        None => {
            return {
                error!("get cluster id failed");
                None
            }
        }
    }

    let mut pinset_should_pin = vec![];

    // TODO: check pins_to_add across pins_to_rm
    let mut review = SyncReview {
        pins_to_add: vec![],
        pins_to_rm: vec![],
    };

    if let Some(cluster_pinset) = cluster_pin_ls().await {
        if let Some(ipfs_pinset) = ipfs_pin_ls().await {
            for cluster_pin in cluster_pinset {
                if cluster_pin.allocations.contains(&id) {
                    pinset_should_pin.push(cluster_pin.cid)
                }
            }

            for cid in &pinset_should_pin {
                if !ipfs_pinset.contains(cid) {
                    review.pins_to_add.push(cid.clone())
                }
            }

            for cid in &ipfs_pinset {
                if !pinset_should_pin.contains(&cid) {
                    review.pins_to_rm.push(cid.clone())
                }
            }

            Some(review)
        } else {
            error!("ipfs pin ls failed");
            None
        }
    } else {
        error!("cluster pin ls failed");
        None
    }
}
