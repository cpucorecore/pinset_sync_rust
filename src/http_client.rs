use crate::settings::S;
use lazy_static::lazy_static;
use log::{debug, error};
use reqwest::Client;
use std::time::Duration;

lazy_static! {
    static ref CLI: Client = Client::new();
    static ref TIMEOUT: Duration = Duration::from_secs(S.proxy.timeout);
}
pub async fn do_post(url: &str) -> Option<String> {
    debug!("http post req: {}", url);

    match CLI.post(url).timeout(*TIMEOUT).send().await {
        Ok(resp) => match resp.text().await {
            Ok(text) => {
                debug!("get http req(post) resp: {}", &text);
                Some(text)
            }
            Err(err) => {
                error!("get http req(post) resp err: {}", err);
                None
            }
        },
        Err(err) => {
            error!("http post req err: {}", err);
            None
        }
    }
}

pub async fn do_get(url: &str) -> Option<String> {
    debug!("http get req: {}", url);

    match CLI.get(url).timeout(*TIMEOUT).send().await {
        Ok(resp) => match resp.text().await {
            Ok(text) => Some(text),
            Err(err) => {
                error!("get http req(get) resp err: {}", err);
                None
            }
        },
        Err(err) => {
            error!("http post req err: {}", err);
            None
        }
    }
}
