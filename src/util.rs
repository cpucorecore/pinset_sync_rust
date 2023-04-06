use get_if_addrs::{get_if_addrs, IfAddr};

pub fn get_local_ip() -> Option<String> {
    let addrs = get_if_addrs().unwrap();
    for addr in addrs {
        if let IfAddr::V4(ipv4_addr) = addr.addr {
            if !ipv4_addr.is_loopback() {
                return Some(ipv4_addr.ip.to_string());
            }
        }
    }

    None
}

// test get_local_ip
#[test]
fn test_get_local_ip() {
    println!("{}", get_local_ip().unwrap());
}
