use crate::http_client::do_post;
use crate::settings::S;
use crate::types::{FileStat, IpfsRepoStat};
use crate::utils::{pare_ipfs_repo_stat, parse_ipfs_file_stat, parse_ipfs_id, parse_ipfs_pin_ls};
use lazy_static::lazy_static;
use log::error;

lazy_static! {
    static ref URL_ID: String = format!("http://{}:{}/api/v0/id", S.proxy.host, S.proxy.ipfs_port);
    static ref URL_REPO_STAT: String = format!(
        "http://{}:{}/api/v0/repo/stat?size-only=false&human=false",
        S.proxy.host, S.proxy.ipfs_port
    );
    static ref URL_PIN_LS: String = format!(
        "http://{}:{}/api/v0/pin/ls?type=recursive",
        S.proxy.host, S.proxy.ipfs_port
    );
}

pub async fn id() -> Option<String> {
    match do_post(&URL_ID).await {
        Some(resp) => parse_ipfs_id(&resp),
        None => None,
    }
}

pub async fn repo_stat() -> Option<IpfsRepoStat> {
    match do_post(&URL_REPO_STAT).await {
        Some(resp) => pare_ipfs_repo_stat(&resp),
        None => None,
    }
}

pub async fn pin_ls() -> Option<Vec<String>> {
    match do_post(&URL_PIN_LS).await {
        Some(resp) => match parse_ipfs_pin_ls(&resp) {
            Some(pins) => Some(pins),
            None => None,
        },
        None => {
            error!("ipfs pin ls failed");
            None
        }
    }
}

pub async fn file_stat(cid: &String) -> Option<FileStat> {
    let url = format!(
        "http://{}:{}/api/v0/files/stat?arg=/ipfs/{}&size=true&with-local=false",
        S.proxy.host, S.proxy.ipfs_port, cid
    );

    match do_post(&url).await {
        Some(resp) => parse_ipfs_file_stat(&resp),
        None => {
            error!("ipfs pin ls failed");
            None
        }
    }
}

pub async fn pin_add(cid: &String) -> Option<String> {
    // curl -X POST "http://127.0.0.1:5001/api/v0/pin/add?arg=/ipfs/QmWgQg88bqTGtFK9Mq7Sq54HKMMFda5htMbkcNdptKMKK3&recursive=true&progress=false"
    let url = format!(
        "http://{}:{}/api/v0/pin/add?arg=/ipfs/{}",
        S.proxy.host, S.proxy.ipfs_port, cid
    );
    do_post(&url).await
}

pub async fn pin_rm(cid: &String) -> Option<String> {
    // curl -X POST "http://127.0.0.1:5001/api/v0/pin/rm?arg=/ipfs/QmWgQg88bqTGtFK9Mq7Sq54HKMMFda5htMbkcNdptKMKK3"
    let url = format!(
        "http://{}:{}/api/v0/pin/rm?arg=/ipfs/{}",
        S.proxy.host, S.proxy.ipfs_port, cid
    );
    do_post(&url).await
}
