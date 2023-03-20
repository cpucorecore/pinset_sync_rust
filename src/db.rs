use crate::settings::SETTINGS;
use crate::types::FileStat;
use kv::Store;
use kv::{Bucket, Config, Json};
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
    static ref PINSET_DB2: Bucket<'static, &'static str, Json<FileStat>> = {
        let pinset_db_name = "pinset";
        DB.bucket::<&str, Json<FileStat>>(Some(pinset_db_name))
            .unwrap()
    };
}

pub fn pinset_set2(key: &str, value: FileStat) {
    match PINSET_DB2.set(&key, &Json(value)) {
        Err(err) => panic!("db set err: {}", err),
        _ => (),
    }
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

pub fn pinset_get2(key: &str) -> Option<FileStat> {
    match PINSET_DB2.get(&key) {
        Err(err) => {
            error!("db get err: {}", err);
            None
        }
        Ok(Some(r)) => Some(r.0),
        Ok(None) => None,
    }
}
