use crate::cmd_executor::{do_cmd, do_daemon_cmd};

pub fn pin_ls() -> Option<Vec<String>> {
    match do_cmd("ipfs", ["pin", "ls", "--type", "recursive"]) {
        None => None,
        Some(output) => {
            let cids: Vec<String> = output
                .rsplit_terminator('\n')
                .map(|line| {
                    line.split_whitespace()
                        .filter(|column| !column.eq(&"recursive"))
                        .collect()
                })
                .collect();
            Some(cids)
        }
    }
}

pub fn gc() -> Option<String> {
    do_cmd("/bin/bash", ["./scripts/gc.sh"])
}

pub fn start_ipfs() -> Option<i32> {
    do_daemon_cmd("ipfs", Box::new(["daemon"]))
}

pub fn stop_ipfs() -> Option<String> {
    do_cmd("/bin/bash", ["./scripts/stop_ipfs.sh"])
}
