use fork::{fork, setsid, Fork};
use log::{debug, error, info};
use std::ffi::OsStr;
use std::process::{exit, Command};

pub fn do_cmd<I, S>(command: &str, args: I) -> Option<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    match Command::new(command).args(args).output() {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(result) => Some(result),
            Err(err) => {
                error!("read cmd execution stdout err: {}", err);
                None
            }
        },
        Err(err) => {
            error!("do cmd err: {}", err);
            None
        }
    }
}

pub fn do_daemon_cmd(command: &str, args: Box<[&str]>) -> Option<i32> {
    match fork() {
        Ok(Fork::Parent(child)) => {
            debug!("parent process continue, new child has pid: {}", child);
            Some(child)
        }
        Ok(Fork::Child) => match setsid() {
            Ok(gid) => {
                debug!("in child process: setsid success, gid: {}", gid);
                match fork() {
                    Ok(Fork::Parent(child)) => {
                        info!(
                            "child process exit, new grandson process has pid: {}",
                            child
                        );
                        exit(0);
                    }
                    Ok(Fork::Child) => {
                        debug!("grandson process continue");
                        let err = exec::Command::new(command).args(args.as_ref()).exec();
                        error!("fork-exec program err: {}", err);
                        Some(0)
                    }
                    Err(ret) => {
                        error!("in child process: fork failed with ret: {}", ret);
                        None
                    }
                }
            }
            Err(err) => {
                error!("in child process: setsid err: {}", err);
                None
            }
        },
        Err(ret) => {
            error!("fork failed with ret: {}", ret);
            None
        }
    }
}
