use crate::cmd_executor::do_cmd;
use log::{debug, error};
use std::str::FromStr;

pub fn get_disk_free_space() -> i64 {
    match do_cmd("/bin/bash", ["./scripts/get_disk_free_space.sh"]) {
        Some(free_space_str) => {
            debug!("free space = {}", free_space_str);
            match i64::from_str(free_space_str.trim()) {
                Ok(free_space) => free_space,
                Err(err) => {
                    error!("parse dist free space string as i64 err: {}", err);
                    -1
                }
            }
        }
        None => -1,
    }
}
