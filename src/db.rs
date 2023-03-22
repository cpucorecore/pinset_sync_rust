use crate::settings::S;
use crate::types::FileStat;
use kv::Store;
use kv::{Bucket, Config, Json};
use lazy_static::lazy_static;
use log::{debug, error};
use std::path::Path;

lazy_static! {
    static ref DB: Store = {
        let path = Path::new(S.db.path.as_str());
        let cfg = Config::new(path);
        Store::new(cfg).unwrap()
    };
    static ref COMMON: Bucket<'static, &'static str, String> =
        DB.bucket::<&str, String>(Some("c")).unwrap();
    static ref FILE_STAT: Bucket<'static, &'static str, Json<FileStat>> =
        DB.bucket::<&str, Json<FileStat>>(Some("p")).unwrap();
}

pub fn save_file_stat(key: &str, value: FileStat) {
    debug!("save_file_stat: k-{}, v-{:?}", key, value);

    match FILE_STAT.set(&key, &Json(value)) {
        Ok(Some(_)) => {
            debug!("db set Ok(Some)");
        }
        Ok(None) => {
            debug!("db set Ok(None)");
        }
        Err(err) => {
            debug!("db set err: {}", err);
        }
    }
}

pub fn get_file_stat(key: &str) -> Option<FileStat> {
    debug!("get_file_stat: k-{}", key);

    match FILE_STAT.get(&key) {
        Ok(Some(v)) => Some(v.0),
        Ok(None) => None,
        Err(err) => {
            error!("db get err: {}", err);
            None
        }
    }
}

pub fn set(key: &str, value: &String) {
    debug!("db set: k-{}, v-{}", key, value);

    match COMMON.set(&key, &value) {
        Err(err) => panic!("db set err: {}", err),
        Ok(Some(v)) => debug!("set return: {:?}", v),
        Ok(None) => (),
    }
}

pub fn get(key: &str) -> Option<String> {
    debug!("db get: k-{}", key);

    match COMMON.get(&key) {
        Err(err) => {
            error!("db get err: {}", err);
            None
        }
        Ok(r) => r,
    }
}
