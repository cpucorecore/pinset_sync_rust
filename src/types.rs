use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ClusterPin {
    pub cid: String,
    pub allocations: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SpaceInfo {
    pub space_pinned: i64,
    pub space_used: i64,
    pub space_ipfs_total: i64,
    pub space_disk_free: i64,
}
