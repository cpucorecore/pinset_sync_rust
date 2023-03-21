use fork::{fork, Fork};
use log::{debug, error};
use std::ffi::OsStr;
use std::process::Command;
use std::str::FromStr;

use crate::types::ClusterPin;
use crate::utils::parse_cluster_state_export_output;

pub fn get_disk_free_space() -> i64 {
    match do_command("/bin/bash", ["./scripts/get_disk_free_space.sh"]) {
        Some(vs) => {
            debug!("vs={}", vs);
            match i64::from_str(vs.trim()) {
                Ok(v) => v,
                Err(err) => {
                    error!("parse i64 err: {}", err);
                    -1
                }
            }
        }
        None => -1,
    }
}

pub fn export_cluster_state() -> Option<Vec<ClusterPin>> {
    let output =
        do_command("ipfs-cluster-service", ["state", "export"]).expect("export state failed");

    parse_cluster_state_export_output(&output)
}

pub fn ipfs_pin_ls() -> Option<Vec<String>> {
    let output = do_command("ipfs", ["pin", "ls", "--type", "recursive"]).expect("pin ls failed");

    let r: Vec<String> = output
        .rsplit_terminator('\n')
        .map(|line| {
            line.split_whitespace()
                .filter(|column| !column.eq(&"recursive"))
                .collect()
        })
        .collect();
    Some(r)
}

pub fn ipfs_gc() -> Option<String> {
    do_command("/bin/bash", ["./scripts/gc.sh"])
}

#[test]
fn test_ipfs_pin_ls() {
    match ipfs_pin_ls() {
        None => {
            error!("err")
        }
        Some(v) => {
            dbg!(v);
        }
    }
}

pub fn start_cluster() -> Option<i32> {
    do_daemon_command("/bin/bash", ["./scripts/daemon_cluster.sh"])
}

pub fn stop_cluster() -> Option<String> {
    do_command("/bin/bash", ["./scripts/stop_cluster.sh"])
}

pub fn start_ipfs() -> Option<i32> {
    do_daemon_command("/bin/bash", ["./scripts/daemon_ipfs.sh"])
}

pub fn stop_ipfs() -> Option<String> {
    do_command("/bin/bash", ["./scripts/stop_ipfs.sh"])
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

fn do_daemon_command<I, S>(command: &str, args: I) -> Option<i32>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    match fork() {
        Ok(Fork::Parent(child)) => {
            println!(
                "Continuing execution in parent process, new child has pid: {}",
                child
            );
            Some(child)
        }
        Ok(Fork::Child) => {
            println!("I'm a new child process");
            let output = Command::new(command)
                .args(args)
                .output()
                .expect("run child process failed");
            println!("status: {}", output.status);
            assert!(output.status.success());
            Some(output.status.code().unwrap())
        }
        Err(_) => {
            println!("Fork failed");
            None
        }
    }
}
