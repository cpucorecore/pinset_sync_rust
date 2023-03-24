use crate::settings::S;
use crate::types_ipfs::FileStat;
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
    static ref K_CLUSTER_ID: String = "id".to_string();
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

pub fn set(key: &str, value: &String) -> Option<String> {
    debug!("db set: [{}]-[{}]", key, value);

    match COMMON.set(&key, &value) {
        Err(err) => {
            error!("db set err: {}", err);
            None
        }
        Ok(Some(old_value)) => Some(old_value),
        Ok(None) => None,
    }
}

pub fn get(key: &str) -> Option<String> {
    debug!("db get key: [{}]", key);

    match COMMON.get(&key) {
        Err(err) => {
            error!("db get err: {}", err);
            None
        }
        Ok(r) => r,
    }
}

pub fn rm(key: &str) -> Option<String> {
    debug!("db rm key: [{}]", key);

    match COMMON.remove(&key) {
        Err(err) => {
            error!("db remove err: {}", err);
            None
        }
        Ok(None) => None,
        Ok(Some(v)) => Some(v),
    }
}

pub fn get_cluster_id() -> Option<String> {
    get(&K_CLUSTER_ID)
}

pub fn save_cluster_id(id: &String) {
    set(&K_CLUSTER_ID, id);
}

pub fn flush() {
    debug!("flush start");
    COMMON.flush().unwrap();
    FILE_STAT.flush().unwrap();
    debug!("flush end");
}
