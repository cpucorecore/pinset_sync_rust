use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct SpaceInfo {
    pub space_pinned: i64,
    pub space_used: i64,
    pub space_ipfs_total: i64,
    pub space_disk_free: i64,
}

#[derive(Debug, Serialize)]
pub struct SyncReview {
    pub cids_to_add: Vec<String>,
    pub cids_to_rm: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct SyncResult {
    pub add_result: HashMap<String, bool>,
    pub rm_result: HashMap<String, bool>,
}
