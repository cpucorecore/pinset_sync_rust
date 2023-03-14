use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct ClusterPin {
    pub cid: String,
    pub allocations: Vec<String>,
}
