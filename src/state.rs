use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum SyncStatus {
    GetSyncReview,
    Syncing,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GcStatus {
    ClusterStopped,
    IpfsStopped,
    ClusterStateExporting,
    ClusterStateExported,
    IpfsPinLs,
    IpfsPinLsFinish,
    DoGc,
    GcFinish,
    Syncing,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Idle,
    Sync(SyncStatus),
    Gc(GcStatus),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub status: Status,
}

impl Default for State {
    fn default() -> Self {
        State {
            status: Status::Idle,
        }
    }
}

impl From<String> for State {
    fn from(s: String) -> Self {
        State {
            status: serde_json::from_str(&s).unwrap(),
        }
    }
}

impl State {
    pub fn info(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
