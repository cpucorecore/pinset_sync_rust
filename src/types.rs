use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct ClusterPin {
    pub cid: String,
    pub allocations: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SpaceInfo {
    pub space_pinned: i64,
    pub space_used: i64,
    pub space_ipfs_total: i64,
    pub space_disk_free: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PinInfo {
    #[serde(rename = "Type")]
    pin_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PinSet {
    #[serde(rename = "Keys")]
    pub keys: HashMap<String, PinInfo>,
}

/*
{
  "Hash": "QmfZF7W8NRnPg6jkvp5Zb42eS1P18kNoAQebg9W2MW5teB",
  "Size": 10485760,
  "CumulativeSize": 10486389,
  "Blocks": 10,
  "Type": "file"
}
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct FileStat {
    #[serde(rename = "Size")]
    pub raw_size: i64,
    #[serde(rename = "CumulativeSize")]
    pub cumulative_size: i64,
}
