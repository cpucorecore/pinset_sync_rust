use crate::http_client::do_get;
use crate::parser::parse_cluster_allocations;
use crate::settings::S;
use crate::types_ipfs_cluster::{Id, Pin};
use lazy_static::lazy_static;
use log::error;
use std::str::FromStr;

lazy_static! {
    static ref URL_ID: String = format!("http://{}:{}/id", S.proxy.host, S.proxy.ipfs_cluster_port);
    static ref URL_ALLOCATIONS: String = format!(
        "http://{}:{}/allocations?filter=all",
        S.proxy.host, S.proxy.ipfs_cluster_port
    );
}

pub async fn allocations() -> Option<Vec<Pin>> {
    match do_get(&URL_ALLOCATIONS).await {
        Some(allocations_str) => Some(parse_cluster_allocations(&allocations_str)),
        None => {
            error!("call cluster api:allocations failed");
            None
        }
    }
}

pub async fn id() -> Option<Id> {
    match do_get(&URL_ID).await {
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
