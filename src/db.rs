use kv::Config;
use kv::Store;
use lazy_static::lazy_static;
use log::{debug, error};
use std::path::Path;
use std::sync::RwLock;

lazy_static! {
    static ref DB: RwLock<Store> = {
        let path = Path::new("db");
        let cfg = Config::new(path);
        let store = Store::new(cfg).unwrap();
        RwLock::new(store)
    };
}

pub fn set(key: &str, value: &String) {
    let bucket = DB.write().unwrap().bucket::<&str, String>(None).unwrap();

    match bucket.set(&key, &value) {
        Err(err) => panic!("db set err: {}", err),
        Ok(Some(v)) => debug!("set return: {:?}", v),
        Ok(None) => (),
    }
}

pub fn get(key: &str) -> Option<String> {
    let bucket = DB.read().unwrap().bucket::<&str, String>(None).unwrap();

    match bucket.get(&key) {
        Ok(Some(value)) => Some(value),
        Ok(None) => None,
        Err(err) => {
            error!("db get err: {}", err);
            None
        }
    }
}
