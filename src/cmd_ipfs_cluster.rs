use crate::cmd_executor::{do_cmd, do_daemon_cmd};
use crate::parser::parse_cluster_allocations;
use crate::types_ipfs_cluster::Pin;
use log::error;

pub fn export_cluster_state() -> Option<Vec<Pin>> {
    match do_cmd("ipfs-cluster-service", ["state", "export"]) {
        Some(output) => Some(parse_cluster_allocations(&output)),
        None => {
            error!("ipfs-cluster-service state export failed");
            None
        }
    }
}

pub fn start_cluster() -> Option<i32> {
    do_daemon_cmd("ipfs-cluster-service", Box::new(["daemon"]))
}

pub fn stop_cluster() -> Option<String> {
    do_cmd("/bin/bash", ["./scripts/stop_cluster.sh"])
}
