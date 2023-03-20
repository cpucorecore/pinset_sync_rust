use crate::settings::SETTINGS;
use kv::Store;
use kv::{Bucket, Config};
use lazy_static::lazy_static;
use log::{debug, error};
use std::path::Path;

lazy_static! {
    static ref DB: Store = {
        let path = Path::new(SETTINGS.db.path.as_str());
        let cfg = Config::new(path);
        Store::new(cfg).unwrap()
    };
    static ref PINSET_DB: Bucket<'static, &'static str, String> = {
        let pinset_db_name = "pinset";
        DB.bucket::<&str, String>(Some(pinset_db_name)).unwrap()
    };
}

pub fn pinset_set(key: &str, value: &String) {
    debug!("k:{}, v:{}", key, value);
    match PINSET_DB.set(&key, &value) {
        Err(err) => panic!("db set err: {}", err),
        Ok(Some(v)) => debug!("set return: {:?}", v),
        Ok(None) => (),
    }
}

pub fn pinset_get(key: &str) -> Option<String> {
    match PINSET_DB.get(&key) {
        Err(err) => {
            error!("db get err: {}", err);
            None
        }
        Ok(r) => r,
    }
}
