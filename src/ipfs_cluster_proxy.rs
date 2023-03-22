use crate::http_client::do_get;
use crate::settings::S;
use crate::types::ClusterPin;
use crate::utils::{parse_cluster_id, parse_cluster_state_export_output};
use lazy_static::lazy_static;
use log::error;

lazy_static! {
    static ref URL_CLUSTER_ID: String =
        format!("http://{}:{}/id", S.proxy.host, S.proxy.ipfs_cluster_port);
    static ref URL_CLUSTER_PIN_LS: String = format!(
        "http://{}:{}/allocations?filter=all",
        S.proxy.host, S.proxy.ipfs_cluster_port
    );
}

pub async fn cluster_pin_ls() -> Option<Vec<ClusterPin>> {
    match do_get(&URL_CLUSTER_PIN_LS).await {
        Some(resp) => match parse_cluster_state_export_output(&resp) {
            Some(pins) => Some(pins),
            None => None,
        },
        None => {
            error!("cluster pin ls failed");
            None
        }
    }
}

pub async fn cluster_id() -> Option<String> {
    match do_get(&URL_CLUSTER_ID).await {
        Some(resp) => match parse_cluster_id(&resp) {
            Some(id) => Some(id),
            None => None,
        },
        None => {
            error!("ipfs id failed");
            None
        }
    }
}
