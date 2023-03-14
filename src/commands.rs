use fork::{fork, Fork};
use std::ffi::OsStr;
use std::process::Command;

use crate::types::ClusterPin;

pub fn export_cluster_state() {
    let output =
        do_command("ipfs-cluster-service", ["state", "export"]).expect("export state failed");

    let state_lines: Vec<&str> = output.split_whitespace().collect();
    for pin in state_lines {
        let cluster_pin: ClusterPin = serde_json::from_str(pin).expect("parse json failed");
        println!("pin: {}:{}", cluster_pin.cid, cluster_pin.allocations[0]);
    }
}

pub fn ipfs_pin_ls() {
    let output = do_command("ipfs", ["pin", "ls", "--type", "recursive"]).expect("pin ls failed");

    let pins: Vec<&str> = output.rsplit_terminator('\n').collect();
    for pin in pins {
        println!("pin: {}", pin);
    }
}

pub fn start_cluster() {
    do_daemon_command("/bin/bash", ["./scripts/daemon_cluster.sh"]);
}

pub fn stop_cluster() {
    let output =
        do_command("/bin/bash", ["./scripts/stop_cluster.sh"]).expect("stop cluster failed");
    println!("stop cluster output: {}", output);
}

pub fn start_ipfs() {
    do_daemon_command("/bin/bash", ["./scripts/daemon_ipfs.sh"]);
}

#[test]
fn test_start_ipfs() {
    start_ipfs()
}

pub fn stop_ipfs() {
    let output = do_command("/bin/bash", ["./scripts/stop_ipfs.sh"]).expect("stop ipfs failed");
    println!("stop ipfs output: {}", output);
}
#[test]
fn test_stop_ipfs() {
    stop_ipfs()
}

fn do_command<I, S>(command: &str, args: I) -> Option<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new(command)
        .args(args)
        .output()
        .expect("do_command failed");

    if output.status.success() {
        Some(String::from_utf8(output.stdout).expect("get stdout err"))
    } else {
        println!(
            "do command err: {}",
            String::from_utf8(output.stderr).expect("get stderr err")
        );
        None
    }
}

fn do_daemon_command<I, S>(command: &str, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    if let Ok(Fork::Child) = fork() {
        Command::new(command)
            .args(args)
            .output()
            .expect("run child process failed");
    }
}
