use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pin {
    #[serde(rename = "Type")]
    pin_type: String,
}

/*
{
  "Keys": {
    "QmQi2oN2JVPpJxykyxfhDqaBTSSrw7vCWFCpwfNUmWTp6t": {
      "Type": "recursive"
    },
    "QmThDREnEA5mvMZSWkuarHTMMA3jJ8zf5soqVzrRGKgvp6": {
      "Type": "recursive"
    },
    "QmZQQYHad2mJHTFC5FLgoKyFPvypbhTVobiXLJBoV5uJqr": {
      "Type": "recursive"
    },
    "QmbS5zRBZaAW7hKspDd22S9ixA2Z5BD8SqH6q1YsTsymxw": {
      "Type": "recursive"
    }
  }
}
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct PinSet {
    #[serde(rename = "Keys")]
    pub keys: HashMap<String, Pin>,
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

impl FromStr for FileStat {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str::<FileStat>(s)
    }
}

/*
curl -X POST "http://192.168.0.85:5001/api/v0/repo/stat" | jq
 */

#[derive(Debug, Deserialize)]
pub struct RepoStat {
    #[serde(rename = "RepoSize")]
    pub repo_size: i64,
    #[serde(rename = "StorageMax")]
    pub storage_max: i64,
}

impl FromStr for RepoStat {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str::<RepoStat>(s)
    }
}

/*
curl -X POST http://127.0.0.1:5001/api/v0/id | jq
 */
#[derive(Debug, Deserialize)]
pub struct Id {
    #[serde(rename = "ID")]
    pub id: String,
}

impl FromStr for Id {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str::<Id>(s)
    }
}
