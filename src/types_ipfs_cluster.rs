use serde_derive::Deserialize;
use std::str::FromStr;

/*
{
  "replication_factor_min": 1,
  "replication_factor_max": 1,
  "name": "",
  "mode": "recursive",
  "shard_size": 0,
  "user_allocations": null,
  "expire_at": "0001-01-01T00:00:00Z",
  "metadata": null,
  "pin_update": null,
  "origins": [],
  "cid": "QmbS5zRBZaAW7hKspDd22S9ixA2Z5BD8SqH6q1YsTsymxw",
  "type": "pin",
  "allocations": [
    "12D3KooWSnEwYadgiJKnrmBDW7s15j3YntU6JPPmTVn28TGBw5y5"
  ],
  "max_depth": -1,
  "reference": null,
  "timestamp": "2022-12-07T16:12:39+08:00"
}
 */
#[derive(Debug, Deserialize)]
pub struct Pin {
    pub cid: String,
    pub allocations: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Id {
    pub id: String,
}

impl FromStr for Id {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str::<Id>(s)
    }
}
