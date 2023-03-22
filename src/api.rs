use crate::commands::{
    cmd_ipfs_pin_ls, export_cluster_state, get_disk_free_space, ipfs_gc, start_cluster, start_ipfs,
    stop_cluster, stop_ipfs,
};
use crate::db;
use crate::ipfs_cluster_proxy::{cluster_id, cluster_pin_ls};
use crate::ipfs_proxy::{ipfs_file_stat, ipfs_pin_add, ipfs_pin_ls, ipfs_pin_rm, ipfs_repo_stat};
use crate::settings::S;
use crate::types::{SpaceInfo, SyncReview};
use actix_web::rt::spawn;
use actix_web::{get, Responder};
use log::{debug, error, info};

async fn collect_ipfs_file_stat(cids: Vec<String>) -> Result<i64, ()> {
    let mut space_pinned = 0_i64;

    for cid in cids {
        match db::get_file_stat(&cid) {
            None => match ipfs_file_stat(&cid).await {
                Some(fs) => {
                    space_pinned += fs.cumulative_size;
                    db::save_file_stat(&cid, fs);
                }
                None => {
                    error!("call api file stat failed");
                }
            },
            Some(fs) => {
                space_pinned += fs.cumulative_size;
            }
        };
    }

    Ok(space_pinned)
}

fn split_worklists(mut cids: Vec<String>) -> Vec<Vec<String>> {
    let mut worklists = vec![];

    let batch_size = cids.len() / S.proxy.worker;
    let _ = (0..S.proxy.worker - 1).map(|_| {
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
        thread_handles.push(spawn(collect_ipfs_file_stat(worklist)));
    }

    for handle in thread_handles {
        space_pinned += handle.await.unwrap().unwrap()
    }

    let stat = ipfs_repo_stat().await.unwrap();

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
            Some(resp) => {
                debug!("ipfs pin add resp:{}", resp);
            }
            None => {
                error!("ipfs pin add failed");
            }
        }
    }

    for cid in &review.pins_to_rm {
        match ipfs_pin_rm(cid).await {
            Some(resp) => {
                debug!("ipfs pin rm resp:{}", resp);
            }
            None => {
                error!("ipfs pin rm failed");
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
