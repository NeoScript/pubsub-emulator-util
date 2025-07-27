use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The struct fields are not comprehensive
/// if you need a certain field that is not included, send a PR
#[derive(Serialize, Deserialize, Debug)]
pub struct Topic {
    pub name: String,
    pub labels: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
pub struct ListTopicsResponse {
    pub topics: Vec<Topic>,
}
