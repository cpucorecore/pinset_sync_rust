use crate::settings::SETTINGS;
use lazy_static::lazy_static;
use reqwest::Client;

lazy_static! {
    static ref HTTP_CLIENT: Client = Client::new();
    static ref URL_IPFS_REPO_STAT: String = format!(
        "http:://{}:{}/api/v0/repo/stat?size-only=false&human=false",
        SETTINGS.dependent_api.host, SETTINGS.dependent_api.ipfs_port
    );
    static ref URL_IPFS_PIN_LS: String = format!(
        "http://{}:{}/api/v0/pin/ls?type=recursive",
        SETTINGS.dependent_api.host, SETTINGS.dependent_api.ipfs_port
    );
}

async fn do_post(url: &str) -> Result<String, reqwest::Error> {
    Ok(HTTP_CLIENT.post(url).send().await?.text().await?)
}

pub async fn ipfs_repo_stat() -> Result<String, reqwest::Error> {
    do_post(&URL_IPFS_REPO_STAT).await
}

pub async fn ipfs_pin_ls() -> Result<String, reqwest::Error> {
    do_post(&URL_IPFS_PIN_LS).await
}

pub async fn file_stat(cid: &String) -> Result<String, reqwest::Error> {
    let url = format!(
        "http://{}:{}/api/v0/files/stat?arg=/ipfs/{}&size=true&with-local=false",
        SETTINGS.dependent_api.host, SETTINGS.dependent_api.ipfs_port, cid
    );
    do_post(&url).await
}

// TODO: add configuration for 127.0.0.1:5001
#[tokio::test]
async fn test_do_post() {
    match do_post(&"http://127.0.0.1:5001/api/v0/repo/stat?size-only=false&human=false").await {
        Ok(msg) => println!("do post response:{}", msg), // TODO: check return "" when have a external http proxy
        Err(err) => println!("do post err:{}", err),
    };
}
