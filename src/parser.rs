use crate::types_ipfs::PinSet;
use crate::types_ipfs_cluster::Pin;
use log::error;

pub fn parse_ipfs_pin_ls(json: &String) -> Option<Vec<String>> {
    match serde_json::from_str::<PinSet>(json) {
        Ok(pinset) => Some(pinset.keys.into_keys().collect::<Vec<String>>()),
        Err(err) => {
            error!("parse ipfs pin ls str:[{}] err: {}", json, err);
            None
        }
    }
}

pub fn parse_cluster_allocations(jsons_str: &String) -> Vec<Pin> {
    let mut pinset: Vec<Pin> = vec![];

    for json in jsons_str.split_whitespace().collect::<Vec<&str>>() {
        match serde_json::from_str::<Pin>(json) {
            Ok(pin) => pinset.push(pin),
            Err(err) => {
                error!("parse cluster pin str:[{}] err: {}", json, err);
            }
        }
    }

    pinset
}
