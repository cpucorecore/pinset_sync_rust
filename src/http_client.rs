async fn do_post(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new(); // TODO: init once
    Ok(client.post(url).send().await?.text().await?)
}

// TODO: add configuration for 127.0.0.1:5001
#[tokio::test]
async fn test_do_post() {
    match do_post(&"http://127.0.0.1:5001/api/v0/repo/stat?size-only=false&human=false").await {
        Ok(msg) => println!("do post response:{}", msg),// TODO: check return "" when have a external http proxy
        Err(err) => println!("do post err:{}", err),
    };
}

pub async fn ipfs_repo_stat() -> Result<String, reqwest::Error> {
    do_post(&"http://127.0.0.1:5001/api/v0/repo/stat?size-only=false&human=false").await
}

pub async fn ipfs_pin_ls() -> Result<String, reqwest::Error> {
    do_post(&"http://127.0.0.1:5001/api/v0/pin/ls?type=recursive").await
}

pub async fn file_stat(cid: &String) -> Result<String, reqwest::Error> {
    let url = format!(
        "http://127.0.0.1:5001/api/v0/files/stat?arg=/ipfs/{}&size=true&with-local=false",
        cid
    );
    do_post(&url).await
}
