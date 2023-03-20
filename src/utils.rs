use crate::types::{ClusterId, ClusterPin, IpfsId, PinSet};
use log::error;

pub fn parse_cluster_state_export_output(output: &String) -> Option<Vec<ClusterPin>> {
    let mut pinset: Vec<ClusterPin> = vec![];

    let state_lines: Vec<&str> = output.split_whitespace().collect();
    for pin in state_lines {
        let cluster_pin: ClusterPin = serde_json::from_str(pin).expect("parse json failed"); // TODO
        pinset.push(cluster_pin)
    }

    Some(pinset)
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

pub fn parse_ipfs_id(data: &String) -> Option<String> {
    match serde_json::from_str::<IpfsId>(data) {
        Ok(id) => Some(id.id),
        Err(err) => {
            error!("parse ipfs id err: {}", err);
            None
        }
    }
}

pub fn parse_cluster_id(data: &String) -> Option<String> {
    match serde_json::from_str::<ClusterId>(data) {
        Ok(id) => Some(id.id),
        Err(err) => {
            error!("parse cluster id err: {}", err);
            None
        }
    }
}
