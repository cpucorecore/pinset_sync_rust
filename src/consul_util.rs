use crate::settings::S;
use crate::util::get_local_ip;
use consulrs::api::check::common::AgentServiceCheckBuilder;
use consulrs::api::service::requests::RegisterServiceRequest;
use consulrs::client::{ConsulClient, ConsulClientSettingsBuilder};
use consulrs::service;
use lazy_static::lazy_static;
use log::{debug, error, info};

lazy_static! {
    static ref API_ENDPOINT: String = format!("http://{}:{}", S.consul.host, S.consul.port);
    static ref CLI: ConsulClient = ConsulClient::new(
        ConsulClientSettingsBuilder::default()
            .address(&*API_ENDPOINT)
            .build()
            .unwrap(),
    )
    .unwrap();
}

async fn register_service(
    name: String,
    address: String,
    port: u64,
    check_path: String,
) -> Option<String> {
    debug!("ip{}", &CLI.settings.address);

    match service::register(
        &*CLI,
        &name,
        Some(
            RegisterServiceRequest::builder()
                .address(&address)
                .port(port)
                .check(
                    AgentServiceCheckBuilder::default()
                        .name("check")
                        .interval("10s")
                        .timeout("3s")
                        .http(format!("http://{}:{}{}", &address, port, check_path))
                        .status("passing")
                        .build()
                        .unwrap(),
                ),
        ),
    )
    .await
    {
        Ok(_) => {
            info!("Service registered successfully!");
            Some(String::from("ok"))
        }
        Err(err) => {
            error!("Failed to register service: {}", err);
            None
        }
    }
}

async fn deregister_service(id: &str) -> Option<String> {
    debug!("ip{}", &CLI.settings.address);
    match service::deregister(&*CLI, id, None).await {
        Ok(_) => {
            info!("Service deregistered successfully!");
            Some(String::from("ok"))
        }
        Err(err) => {
            error!("Failed to deregister service: {}", err);
            None
        }
    }
}

async fn list_services() -> Option<String> {
    debug!("ip{}", &CLI.settings.address);
    match service::list(&*CLI, None).await {
        Ok(services) => {
            info!("Service: {:?}", services);
            Some(String::from("ok"))
        }
        Err(err) => {
            error!("Failed to list services: {}", err);
            None
        }
    }
}

pub async fn register_cluster() -> Option<String> {
    let service_address = if let Some(ip) = get_local_ip() {
        ip
    } else {
        return None;
    };

    register_service(
        String::from("cluster"),
        service_address,
        9094_u64,
        String::from("/id"),
    )
    .await
}

// test register_cluster()
#[tokio::test]
async fn test_register_cluster() {
    log4rs::init_file("conf/log.yml", Default::default()).unwrap();
    register_cluster().await;
}

// test deregister_service()
#[tokio::test]
async fn test_derigister_service() {
    deregister_service("cluster").await;
}

// test list_services()
#[tokio::test]
async fn test_list_services() {
    log4rs::init_file("conf/log.yml", Default::default()).unwrap();
    list_services().await;
}
