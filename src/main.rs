use fork::{daemon, Fork};
use std::{process::Command};

fn main() {
    let result = Command::new("ipfs-cluster-service")
    .args(["state", "export"])
    .output()
    .expect("failed to execute command");
println!("status: {}", result.status);
// println!("stdout: {}", String::from_utf8_lossy(&result.stdout));
// println!("stderr: {}", String::from_utf8_lossy(&result.stderr));

let d = String::from_utf8_lossy(&result.stdout);
let state :Vec<&str> = d.split_whitespace().collect();
for pin in state {
    // println!("status: {}", pin);
    let parsed = json::parse(pin).unwrap();
    println!("status: {}", parsed);
}


    println!("Hello, world!");

    // fork and detach
    // if let Ok(Fork::Child) = daemon(false, false) {
    //     Command::new("/bin/bash")
    //         .arg("/Users/liukunyu/test.sh")
    //         .output()
    //         .expect("failed to execute process");
    //  }

     // 

}
