use fork::{daemon, fork, Fork};
use log::{error, info};
use std::ffi::OsStr;
use std::process::Command;

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
            info!(
                "Continuing execution in parent process, new child has pid: {}",
                child
            );
            Some(child)
        }
        Ok(Fork::Child) => {
            if let Ok(Fork::Child) = daemon(false, false) {
                let err = exec::Command::new(command).args(args.as_ref()).exec();
                error!("fork-fork-exec program err: {}", err);
            }
            Some(0)
        }
        Err(ret) => {
            error!("fork failed with ret: {}", ret);
            None
        }
    }
}
