use lazy_static::lazy_static;
use log::{debug, error};
use reqwest::Client;

lazy_static! {
    static ref CLI: Client = Client::new();
}
pub async fn do_post(url: &str) -> Option<String> {
    debug!("http post req: {}", url);

    match CLI.post(url).send().await {
        Ok(resp) => match resp.text().await {
            Ok(text) => Some(text),
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

    match CLI.get(url).send().await {
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
