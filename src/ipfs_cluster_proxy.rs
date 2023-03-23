use crate::http_client::do_get;
use crate::settings::S;
use crate::types::ClusterPin;
use crate::utils::{parse_cluster_allocations, parse_cluster_id};
use lazy_static::lazy_static;
use log::error;

lazy_static! {
    static ref URL_ID: String = format!("http://{}:{}/id", S.proxy.host, S.proxy.ipfs_cluster_port);
    static ref URL_ALLOCATIONS: String = format!(
        "http://{}:{}/allocations?filter=all",
        S.proxy.host, S.proxy.ipfs_cluster_port
    );
}

pub async fn allocations() -> Option<Vec<ClusterPin>> {
    match do_get(&URL_ALLOCATIONS).await {
        Some(allocations_str) => match parse_cluster_allocations(&allocations_str) {
            Some(allocations) => Some(allocations),
            None => {
                error!(
                    "parse cluster allocations failed, the allocations_str: [{}]",
                    allocations_str
                );
                None
            }
        },
        None => {
            error!("call cluster api:allocations failed");
            None
        }
    }
}

pub async fn id() -> Option<String> {
    match do_get(&URL_ID).await {
        Some(id_str) => match parse_cluster_id(&id_str) {
            Some(id) => Some(id),
            None => {
                error!("parse cluster id failed, the id_str: [{}]", id_str);
                None
            }
        },
        None => {
            error!("call cluster api:id failed");
            None
        }
    }
}
