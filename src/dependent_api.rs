use crate::settings::SETTINGS;
use crate::types::ClusterPin;
use crate::utils::{
    parse_cluster_id, parse_cluster_state_export_output, parse_ipfs_id, parse_ipfs_pin_ls,
};
use lazy_static::lazy_static;
use log::error;
use reqwest::Client;

lazy_static! {
    static ref HTTP_CLIENT: Client = Client::new();
    static ref URL_IPFS_REPO_STAT: String = format!(
        "http://{}:{}/api/v0/repo/stat?size-only=false&human=false",
        SETTINGS.dependent_api.host, SETTINGS.dependent_api.ipfs_port
    );
    static ref URL_IPFS_PIN_LS: String = format!(
        "http://{}:{}/api/v0/pin/ls?type=recursive",
        SETTINGS.dependent_api.host, SETTINGS.dependent_api.ipfs_port
    );
    static ref URL_IPFS_ID: String = format!(
        "http://{}:{}/api/v0/id",
        SETTINGS.dependent_api.host, SETTINGS.dependent_api.ipfs_port
    );
    static ref URL_CLUSTER_ID: String = format!(
        "http://{}:{}/id",
        SETTINGS.dependent_api.host, SETTINGS.dependent_api.ipfs_cluster_port
    );
    static ref URL_CLUSTER_PIN_LS: String = format!(
        "http://{}:{}/allocations?filter=all",
        SETTINGS.dependent_api.host, SETTINGS.dependent_api.ipfs_cluster_port
    );
}

async fn do_post(url: &str) -> Result<String, reqwest::Error> {
    Ok(HTTP_CLIENT.post(url).send().await?.text().await?)
}

async fn do_get(url: &str) -> Result<String, reqwest::Error> {
    Ok(HTTP_CLIENT.get(url).send().await?.text().await?)
}

pub async fn ipfs_repo_stat() -> Result<String, reqwest::Error> {
    do_post(&URL_IPFS_REPO_STAT).await
}

pub async fn ipfs_pin_ls() -> Option<Vec<String>> {
    match do_post(&URL_IPFS_PIN_LS).await {
        Ok(resp) => match parse_ipfs_pin_ls(&resp) {
            Some(pins) => Some(pins),
            None => None,
        },
        Err(err) => {
            error!("ipfs pin ls failed: {}", err);
            None
        }
    }
}

pub async fn ipfs_file_stat(cid: &String) -> Result<String, reqwest::Error> {
    let url = format!(
        "http://{}:{}/api/v0/files/stat?arg=/ipfs/{}&size=true&with-local=false",
        SETTINGS.dependent_api.host, SETTINGS.dependent_api.ipfs_port, cid
    );
    do_post(&url).await
}

pub async fn ipfs_pin_add(cid: &String) -> Result<String, reqwest::Error> {
    // curl -X POST "http://127.0.0.1:5001/api/v0/pin/add?arg=/ipfs/QmWgQg88bqTGtFK9Mq7Sq54HKMMFda5htMbkcNdptKMKK3&recursive=true&progress=false"
    let url = format!(
        "http://{}:{}/api/v0/pin/add?arg=/ipfs/{}",
        SETTINGS.dependent_api.host, SETTINGS.dependent_api.ipfs_port, cid
    );
    do_post(&url).await
}

pub async fn ipfs_pin_rm(cid: &String) -> Result<String, reqwest::Error> {
    // curl -X POST "http://127.0.0.1:5001/api/v0/pin/rm?arg=/ipfs/QmWgQg88bqTGtFK9Mq7Sq54HKMMFda5htMbkcNdptKMKK3"
    let url = format!(
        "http://{}:{}/api/v0/pin/rm?arg=/ipfs/{}",
        SETTINGS.dependent_api.host, SETTINGS.dependent_api.ipfs_port, cid
    );
    do_post(&url).await
}

pub async fn cluster_pin_ls() -> Option<Vec<ClusterPin>> {
    match do_get(&URL_CLUSTER_PIN_LS).await {
        Ok(resp) => match parse_cluster_state_export_output(&resp) {
            Some(pins) => Some(pins),
            None => None,
        },
        Err(err) => {
            error!("cluster pin ls failed: {}", err);
            None
        }
    }
}

pub async fn cluster_id() -> Option<String> {
    match do_get(&URL_CLUSTER_ID).await {
        Ok(resp) => match parse_cluster_id(&resp) {
            Some(id) => Some(id),
            None => None,
        },
        Err(err) => {
            error!("ipfs id failed: {}", err);
            None
        }
    }
}

pub async fn ipfs_id() -> Option<String> {
    match do_post(&URL_IPFS_ID).await {
        Ok(resp) => match parse_ipfs_id(&resp) {
            Some(id) => Some(id),
            None => None,
        },
        Err(err) => {
            error!("ipfs id failed: {}", err);
            None
        }
    }
}
