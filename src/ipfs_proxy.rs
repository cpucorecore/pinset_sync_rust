use crate::http_client::do_post;
use crate::parser::parse_ipfs_pin_ls;
use crate::settings::S;
use crate::types_ipfs::{FileStat, Id, RepoStat};
use lazy_static::lazy_static;
use log::error;
use std::str::FromStr;

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

pub async fn id() -> Option<Id> {
    match do_post(&URL_ID).await {
        Some(id_str) => match Id::from_str(&id_str) {
            Ok(id_obj) => Some(id_obj),
            Err(err) => {
                error!("Id from str:[{}] err: {}", id_str, err);
                None
            }
        },
        None => {
            error!("call ipfs api:id failed");
            None
        }
    }
}

pub async fn repo_stat() -> Option<RepoStat> {
    match do_post(&URL_REPO_STAT).await {
        Some(repo_stat_str) => match RepoStat::from_str(&repo_stat_str) {
            Ok(repo_stat_obj) => Some(repo_stat_obj),
            Err(err) => {
                error!("RepoStat from str:[{}] err: {}", repo_stat_str, err);
                None
            }
        },
        None => {
            error!("call ipfs api:repo_stat failed");
            None
        }
    }
}

pub async fn pin_ls() -> Option<Vec<String>> {
    match do_post(&URL_PIN_LS).await {
        Some(pin_ls_str) => match parse_ipfs_pin_ls(&pin_ls_str) {
            Some(pins) => Some(pins),
            None => None,
        },
        None => {
            error!("call ipfs api:pin_ls failed");
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
        Some(file_stat_str) => match FileStat::from_str(&file_stat_str) {
            Ok(file_stat_obj) => Some(file_stat_obj),
            Err(err) => {
                error!("FileStat from str:[{}] err: {}", file_stat_str, err);
                None
            }
        },
        None => {
            error!("call ipfs api:file_stat failed");
            None
        }
    }
}

pub async fn pin_add(cid: &String) -> Option<String> {
    let url = format!(
        "http://{}:{}/api/v0/pin/add?arg=/ipfs/{}",
        S.proxy.host, S.proxy.ipfs_port, cid
    );
    do_post(&url).await
}

pub async fn pin_rm(cid: &String) -> Option<String> {
    let url = format!(
        "http://{}:{}/api/v0/pin/rm?arg=/ipfs/{}",
        S.proxy.host, S.proxy.ipfs_port, cid
    );
    do_post(&url).await
}
