use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
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

// impl kv::Value for FileStat {
//     fn to_raw_value(&self) -> Result<Raw, Error> {
//         match serde_json::to_string(self) {
//             Ok(v) => Ok(Raw::from(v.as_str())),
//             Err(err) => Err(Error::Message(err.to_string())),
//         }
//     }
//
//     fn from_raw_value(r: Raw) -> Result<Self, Error> {
//         match serde_json::from_value(r) {
//             Ok(v) => Ok(v),
//             Err(err) => Err(Error::Message(err.to_string())),
//         }
//     }
// }

/*
curl -X POST "http://192.168.0.85:5001/api/v0/repo/stat"
{
  "RepoSize": 314830983748,
  "StorageMax": 644245094400,
  "NumObjects": 360293,
  "RepoPath": "/ac/store/.ipfs",
  "Version": "fs-repo@12"
}
 */

#[derive(Debug, Deserialize)]
pub struct IpfsRepoStat {
    #[serde(rename = "RepoSize")]
    pub repo_size: i64,
    #[serde(rename = "StorageMax")]
    pub storage_max: i64,
}

/*
curl -X POST http://127.0.0.1:5001/api/v0/id |jq
{
  "ID": "12D3KooWSJMSyjLGFJzkHRB7x7xp8jVfxCbAiT5oSNmEnQY6QbqB",
  "PublicKey": "CAESIPTmcHNtu0E3HmAoy1W87GwiTJiA1oIsH9r68e7vbGw6",
  "Addresses": [
    "/ip4/127.0.0.1/tcp/4001/p2p/12D3KooWSJMSyjLGFJzkHRB7x7xp8jVfxCbAiT5oSNmEnQY6QbqB",
    "/ip4/192.168.0.86/tcp/4001/p2p/12D3KooWSJMSyjLGFJzkHRB7x7xp8jVfxCbAiT5oSNmEnQY6QbqB",
    "/ip6/::1/tcp/4001/p2p/12D3KooWSJMSyjLGFJzkHRB7x7xp8jVfxCbAiT5oSNmEnQY6QbqB"
  ],
  "AgentVersion": "kubo/0.17.0/",
  "ProtocolVersion": "ipfs/0.1.0",
  "Protocols": [
    "/ipfs/bitswap",
    "/ipfs/bitswap/1.0.0",
    "/ipfs/bitswap/1.1.0",
    "/ipfs/bitswap/1.2.0",
    "/ipfs/id/1.0.0",
    "/ipfs/id/push/1.0.0",
    "/ipfs/lan/kad/1.0.0",
    "/ipfs/ping/1.0.0",
    "/libp2p/autonat/1.0.0",
    "/libp2p/circuit/relay/0.1.0",
    "/libp2p/circuit/relay/0.2.0/stop",
    "/p2p/id/delta/1.0.0",
    "/x/"
  ]
}
 */
#[derive(Debug, Deserialize)]
pub struct IpfsId {
    #[serde(rename = "ID")]
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct ClusterId {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct SyncReview<'a> {
    pub pins_to_add: Vec<&'a String>,
    pub pins_to_rm: Vec<&'a String>,
}
