use crate::types::{ClusterId, ClusterPin, FileStat, IpfsId, IpfsRepoStat, PinSet};
use log::error;

pub fn parse_ipfs_id(data: &String) -> Option<String> {
    match serde_json::from_str::<IpfsId>(data) {
        Ok(ipfs_id) => Some(ipfs_id.id),
        Err(err) => {
            error!("parse ipfs id err: {}", err);
            None
        }
    }
}

pub fn pare_ipfs_repo_stat(data: &String) -> Option<IpfsRepoStat> {
    match serde_json::from_str::<IpfsRepoStat>(data) {
        Ok(stat) => Some(stat),
        Err(err) => {
            error!("parse ipfs repo stat err: {}", err);
            None
        }
    }
}

pub fn parse_ipfs_pin_ls(data: &String) -> Option<Vec<String>> {
    match serde_json::from_str::<PinSet>(data) {
        Ok(pinset) => {
            let cids: Vec<String> = pinset.keys.into_keys().collect();
            Some(cids)
        }
        Err(err) => {
            error!("parse ipfs pin ls err: {}", err);
            None
        }
    }
}

pub fn parse_ipfs_file_stat(data: &String) -> Option<FileStat> {
    match serde_json::from_str::<FileStat>(data) {
        Ok(file_stat) => Some(file_stat),
        Err(err) => {
            error!("parse ipfs file stat err: {}", err);
            None
        }
    }
}

pub fn parse_cluster_allocations(output: &String) -> Option<Vec<ClusterPin>> {
    let mut pinset: Vec<ClusterPin> = vec![];

    let state_lines: Vec<&str> = output.split_whitespace().collect();
    for pin in state_lines {
        let cluster_pin: ClusterPin = serde_json::from_str(pin).expect("parse json failed"); // TODO
        pinset.push(cluster_pin)
    }

    Some(pinset)
}

pub fn parse_cluster_id(data: &String) -> Option<String> {
    match serde_json::from_str::<ClusterId>(data) {
        Ok(cluster_id) => Some(cluster_id.id),
        Err(err) => {
            error!("parse cluster id err: {}", err);
            None
        }
    }
}
