use crate::cmd_common::get_disk_free_space;
use crate::cmd_ipfs;
use crate::cmd_ipfs_cluster;
use crate::db;
use crate::ipfs_cluster_proxy;
use crate::ipfs_proxy;
use crate::settings::S;
use crate::types::{SpaceInfo, SyncResult, SyncReview};
use crate::types_ipfs_cluster::Pin;
use actix_web::rt::spawn;
use actix_web::{get, Responder};
use log::{debug, error, info};
use std::time::Duration;
use tokio::time::sleep;

#[get("/sync_review")]
pub async fn sync_review() -> impl Responder {
    match get_sync_review().await {
        Some(review) => serde_json::to_string(&review).unwrap(),
        None => "call api failed".to_string(),
    }
}

async fn get_sync_review() -> Option<SyncReview> {
    match (
        ipfs_cluster_proxy::allocations().await,
        ipfs_proxy::pin_ls().await,
    ) {
        (Some(cluster_pinset), Some(ipfs_pinset)) => {
            let id = db::get_cluster_id().unwrap();
            let mut cids_duty = vec![];
            for pin in cluster_pinset {
                if pin.allocations.contains(&id) {
                    cids_duty.push(pin.cid)
                }
            }

            let mut review = SyncReview {
                cids_to_add: vec![],
                cids_to_rm: vec![],
            };

            for cid in &cids_duty {
                if !ipfs_pinset.contains(cid) {
                    review.cids_to_add.push(cid.clone())
                }
            }

            for cid in &ipfs_pinset {
                if !cids_duty.contains(&cid) {
                    review.cids_to_rm.push(cid.clone())
                }
            }

            Some(review)
        }
        _ => {
            error!("call api failed");
            None
        }
    }
}

#[get("/sync")]
pub async fn sync() -> impl Responder {
    // TODO: 1. sync lock; 2. sync status;
    match get_sync_review().await {
        Some(review) => {
            let sync_result = do_sync(&review).await;
            serde_json::to_string(&sync_result).unwrap()
        }
        None => "call api failed".to_string(),
    }
}

async fn do_sync(review: &SyncReview) -> SyncResult {
    // TODO: multiple thread do it, with flow control

    let mut result = SyncResult {
        add_result: Default::default(),
        rm_result: Default::default(),
    };

    for cid in &review.cids_to_add {
        match ipfs_proxy::pin_add(cid).await {
            Some(resp) => {
                debug!("ipfs pin add resp:{}", resp);
                result.add_result.insert(cid.clone(), true);
            }
            None => {
                error!("ipfs pin add failed");
                result.add_result.insert(cid.clone(), false);
            }
        }
    }

    for cid in &review.cids_to_rm {
        match ipfs_proxy::pin_rm(cid).await {
            Some(resp) => {
                debug!("ipfs pin rm resp:{}", resp);
                result.rm_result.insert(cid.clone(), true);
            }
            None => {
                error!("ipfs pin rm failed");
                result.rm_result.insert(cid.clone(), false);
            }
        }
    }

    result
}

#[get("/space_info")]
pub async fn space_info() -> impl Responder {
    serde_json::to_string(&get_space_info().await).unwrap()
}

async fn get_space_info() -> SpaceInfo {
    let mut space_pinned = 0_i64;
    if let Some(cids) = ipfs_proxy::pin_ls().await {
        debug!("cids: {:?}", cids);
        let mut handles = vec![];
        let worklist = split_worklist(cids);
        for work in worklist {
            handles.push(spawn(collect_ipfs_file_stat(work)));
        }

        for handle in handles {
            space_pinned += handle.await.unwrap();
        }
    } else {
        space_pinned = -1;
    }

    let mut space_used = -1;
    let mut space_ipfs_total = -1;
    match ipfs_proxy::repo_stat().await {
        None => {}
        Some(stat) => {
            space_used = stat.repo_size;
            space_ipfs_total = stat.storage_max;
        }
    }

    SpaceInfo {
        space_pinned,
        space_used,
        space_ipfs_total,
        space_disk_free: get_disk_free_space(),
    }
}

fn cross_pinset(cluster_pinset: &Vec<Pin>, ipfs_pinset: &Vec<String>) -> SyncReview {
    let mut review = SyncReview {
        cids_to_add: vec![],
        cids_to_rm: vec![],
    };

    let mut pinset_should_pin = vec![];

    let cluster_id = db::get_cluster_id().unwrap();
    for cluster_pin in cluster_pinset {
        if cluster_pin.allocations.contains(&cluster_id) {
            pinset_should_pin.push(cluster_pin.cid)
        }
    }

    for cid in &pinset_should_pin {
        if !ipfs_pinset.contains(cid) {
            review.cids_to_add.push(cid.clone())
        }
    }

    for cid in &ipfs_pinset {
        if !pinset_should_pin.contains(&cid) {
            review.cids_to_rm.push(cid.clone())
        }
    }
}

// TODO: 1. global status for query; 2. session; 3. gc lock
#[get("/gc")]
pub async fn gc() -> impl Responder {
    // TODO: detect ipfs and ipfs cluster alive

    if let None = cmd_ipfs_cluster::stop_cluster() {
        error!("stop cluster failed");
        return "stop cluster failed";
    }
    info!("cluster stopped");

    if let None = cmd_ipfs::stop_ipfs() {
        error!("stop ipfs failed");
        cmd_ipfs_cluster::start_cluster(); // TODO: check success?
        return "stop ipfs failed";
    }
    info!("ipfs stopped");

    let cluster_pinset = cmd_ipfs_cluster::export_cluster_state();
    info!("cluster state export finish");

    match cmd_ipfs::pin_ls() {
        None => {
            cmd_ipfs::start_ipfs();
            cmd_ipfs_cluster::start_cluster();

            error!("ipfs pin ls failed");
            return "ipfs pin ls failed";
        }
        Some(ipfs_pinset) => {
            info!("ipfs pin ls finish");

            match cmd_ipfs::gc() {
                None => {
                    cmd_ipfs::start_ipfs();
                    cmd_ipfs_cluster::start_cluster();

                    error!("ipfs gc failed");
                    return "ipfs gc failed";
                }
                Some(_) => {
                    info!("ipfs gc finish");

                    match cmd_ipfs::start_ipfs() {
                        None => {
                            error!("ipfs start failed");
                            return "ipfs start failed";
                        }
                        Some(ipfs_pid) => {
                            info!("ipfs started, pid: {}", ipfs_pid);

                            let mut pinset_should_pin = vec![];
                            let mut review = SyncReview {
                                cids_to_add: vec![],
                                cids_to_rm: vec![],
                            };

                            let cluster_id = db::get_cluster_id().unwrap();
                            for cluster_pin in cluster_pinset {
                                if cluster_pin.allocations.contains(&cluster_id) {
                                    pinset_should_pin.push(cluster_pin.cid)
                                }
                            }

                            for cid in &pinset_should_pin {
                                if !ipfs_pinset.contains(cid) {
                                    review.cids_to_add.push(cid.clone())
                                }
                            }

                            for cid in &ipfs_pinset {
                                if !pinset_should_pin.contains(&cid) {
                                    review.cids_to_rm.push(cid.clone())
                                }
                            }

                            sleep(Duration::from_secs(5)).await; // wait ipfs startup. TODO: detect by loop api call
                            do_sync(&review).await;
                            debug!("do_sync finish");

                            return match cmd_ipfs_cluster::start_cluster() {
                                None => {
                                    error!("start cluster failed");
                                    "start cluster failed"
                                }
                                Some(cluster_pid) => {
                                    info!("cluster started, pid: {}", cluster_pid);
                                    "ok"
                                }
                            };
                        }
                    }
                }
            }
        }
    }
}

fn split_worklist(mut cids: Vec<String>) -> Vec<Vec<String>> {
    let mut worklist = vec![];

    let cids_len = cids.len();
    if cids_len == 0 {
        return worklist;
    }

    if cids_len <= S.proxy.worker {
        worklist.push(cids);
        return worklist;
    }

    let work_size = cids.len() / S.proxy.worker;
    (0..S.proxy.worker - 1)
        .map(|_| {
            let batch: Vec<String> = cids.drain(0..work_size).collect();
            worklist.push(batch);
        })
        .count();

    let last_batch: Vec<String> = cids.drain(0..cids.len()).collect();
    worklist.push(last_batch);

    worklist
}

async fn collect_ipfs_file_stat(cids: Vec<String>) -> i64 {
    let mut space_pinned = 0_i64;

    for cid in cids {
        match db::get_file_stat(&cid) {
            None => {
                if let Some(fs) = ipfs_proxy::file_stat(&cid).await {
                    space_pinned += fs.cumulative_size;
                    db::save_file_stat(&cid, fs);
                }
            }
            Some(fs) => {
                space_pinned += fs.cumulative_size;
            }
        };
    }

    space_pinned
}
