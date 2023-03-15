use log::info;

fn main() {
    log4rs::init_file("conf/log.yml", Default::default()).unwrap();
    let _rt = tokio::runtime::Runtime::new().unwrap();
    info!("main finished");
}
