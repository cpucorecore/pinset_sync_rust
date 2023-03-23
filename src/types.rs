use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SpaceInfo {
    pub space_pinned: i64,
    pub space_used: i64,
    pub space_ipfs_total: i64,
    pub space_disk_free: i64,
}

#[derive(Debug, Serialize)]
pub struct SyncReview {
    pub pins_to_add: Vec<String>,
    pub pins_to_rm: Vec<String>,
}
