use log::error;

use reqwest::header::HeaderValue;
use serde_json::json;

pub async fn consul_register_service() {
    let api_endpoint = "http://192.168.0.85:8500/v1";

    let client = reqwest::Client::new();

    let service_name = "cluster";
    let service_address = "192.168.0.85";
    let service_port = 19094;
    let service_meta = json!({"version": "1.0.0"});

    let register_request = client
        .put(&format!("{}/agent/service/register", api_endpoint))
        .header("Content-Type", HeaderValue::from_static("application/json"))
        .json(&json!({
            "ID": service_name,
            "Name": service_name,
            "Address": service_address,
            "Port": service_port,
            "Meta": service_meta
        }));

    // Send the request and handle the response
    let response = register_request.send().await.unwrap();
    if response.status().is_success() {
        println!("Service registered successfully!");
    } else {
        error!("Failed to register service: {}", response.status());
    }
}

#[tokio::test]
async fn test_consul_register_service() {
    consul_register_service().await;
}
