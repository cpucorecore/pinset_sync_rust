pub enum SyncStatus {
    ClusterStateExporting,
    IpfsPinLs,
    Sync,
}

pub enum GcStatus {
    ClusterStopped,
    IpfsStopped,
    ClusterStateExporting,
    IpfsPinLs,
    DoGc,
    Sync,
    Finish,
}

pub enum Status {
    Idle,
    Sync(SyncStatus),
    Gc(GcStatus),
}
