use crate::cmd_executor::{do_cmd, do_daemon_cmd};
use crate::parser::parse_cluster_allocations;
use crate::types_ipfs_cluster::Pin;

pub fn export_cluster_state() -> Vec<Pin> {
    let output = do_cmd("ipfs-cluster-service", ["state", "export"]).expect("export state failed");
    parse_cluster_allocations(&output)
}

pub fn start_cluster() -> Option<i32> {
    // do_daemon_command("/bin/bash", ["./scripts/daemon_cluster.sh"])
    do_daemon_cmd("ipfs-cluster-service", Box::new(["daemon"]))
}

pub fn stop_cluster() -> Option<String> {
    do_cmd("/bin/bash", ["./scripts/stop_cluster.sh"])
}
