use crate::http_client::do_get;
use crate::parser::parse_cluster_allocations;
use crate::settings::S;
use crate::types_ipfs_cluster::{Id, Pin};
use lazy_static::lazy_static;
use log::error;
use std::str::FromStr;
use tokio::time::{sleep, Duration};

lazy_static! {
    static ref URL_ID: String = format!("http://{}:{}/id", S.proxy.host, S.proxy.ipfs_cluster_port);
    static ref URL_ALLOCATIONS: String = format!(
        "http://{}:{}/allocations?filter=all",
        S.proxy.host, S.proxy.ipfs_cluster_port
    );
}

pub async fn allocations() -> Option<Vec<Pin>> {
    match do_get(&URL_ALLOCATIONS, S.proxy.timeout_allocations).await {
        Some(allocations_str) => Some(parse_cluster_allocations(&allocations_str)),
        None => {
            error!("call cluster api:allocations failed");
            None
        }
    }
}

pub async fn id() -> Option<Id> {
    match do_get(&URL_ID, S.proxy.timeout).await {
        Some(id_str) => match Id::from_str(&id_str) {
            Ok(id_obj) => Some(id_obj),
            Err(err) => {
                error!("Id from str:[{}] err: {}", id_str, err);
                None
            }
        },
        None => {
            error!("call cluster api:id failed");
            None
        }
    }
}

pub async fn alive() -> bool {
    match id().await {
        Some(_) => true,
        None => false,
    }
}

pub async fn wait_alive(max_retry: i32) -> bool {
    let mut cnt = 1;
    return loop {
        if alive().await {
            break true;
        } else {
            if cnt >= max_retry {
                break false;
            } else {
                cnt += 1;
            }
        }
        sleep(Duration::from_secs(1)).await;
    };
}
