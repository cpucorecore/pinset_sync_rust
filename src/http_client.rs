pub async fn do_post() -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    Ok(client
        .post("http://192.168.0.85:5001/api/v0/repo/stat?size-only=false&human=false")
        .send()
        .await?
        .text()
        .await?)
}

#[tokio::test]
async fn test_do_post() {
    match do_post().await {
        Ok(msg) => println!("do post response:{}", msg),
        Err(err) => println!("do post err:{}", err),
    };
}
