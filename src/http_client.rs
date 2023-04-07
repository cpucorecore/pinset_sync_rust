use crate::settings::S;
use lazy_static::lazy_static;
use log::{debug, error};
use reqwest::Client;
use std::time::Duration;

lazy_static! {
    static ref CLI: Client = Client::new();
    static ref TIMEOUT: Duration = Duration::from_secs(S.proxy.timeout);
}

pub async fn do_post(url: &str, timeout: u64) -> Option<String> {
    debug!("post req: {}", url);

    match CLI
        .post(url)
        .timeout(Duration::from_secs(timeout))
        .send()
        .await
    {
        Ok(resp) => match resp.text().await {
            Ok(text) => {
                debug!("get post resp: {}", &text);
                Some(text)
            }
            Err(err) => {
                error!("get post resp err: {}", err);
                None
            }
        },
        Err(err) => {
            error!("post err: {}", err);
            None
        }
    }
}

pub async fn do_get(url: &str, timeout: u64) -> Option<String> {
    debug!("http get req: {}", url);

    match CLI
        .get(url)
        .basic_auth(
            &S.proxy.ipfs_cluster_user,
            Some(&S.proxy.ipfs_cluster_password),
        )
        .timeout(Duration::from_secs(timeout))
        .send()
        .await
    {
        Ok(resp) => match resp.text().await {
            Ok(text) => {
                debug!("get http req(get) resp: {}", &text);
                Some(text)
            }
            Err(err) => {
                error!("get http req(get) resp err: {}", err);
                None
            }
        },
        Err(err) => {
            error!("http get req err: {}", err);
            None
        }
    }
}
