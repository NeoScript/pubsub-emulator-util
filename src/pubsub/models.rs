use base64::{Engine as _, engine::general_purpose::STANDARD};
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub client: Arc<Client>,
    pub project_id: String,
    pub host: String,
}

/// The struct fields are not comprehensive
/// if you need a certain field that is not included, send a PR
#[derive(Serialize, Deserialize, Debug)]
pub struct Topic {
    pub name: String,
    pub labels: Option<HashMap<String, String>>,
}

impl Topic {
    pub fn full_path(&self, project_id: &str) -> String {
        let name = &self.name;
        format!("projects/{project_id}/topics/{name}")
    }
}

#[derive(Deserialize, Debug)]
pub struct ListTopicsResponse {
    pub topics: Vec<Topic>,
}

#[derive(Serialize, Debug)]
pub struct PublishMessagePayload {
    messages: Vec<SendablePubsubMessage>,
}

#[derive(Debug)]
pub struct RawPubsubMessageToSend {
    // NOTE: this data payload need to be encoded to b64 before sending out
    data: Option<String>,
    attributes: Option<HashMap<String, String>>,
    ordering_key: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendablePubsubMessage {
    data: Option<String>,
    attributes: Option<HashMap<String, String>>,
    ordering_key: Option<String>,
}

impl From<&RawPubsubMessageToSend> for SendablePubsubMessage {
    fn from(raw: &RawPubsubMessageToSend) -> Self {
        Self {
            data: raw.data.as_ref().map(|payload| STANDARD.encode(payload)),
            attributes: raw.attributes.clone(),
            ordering_key: raw.ordering_key.clone(),
        }
    }
}

impl From<RawPubsubMessageToSend> for SendablePubsubMessage {
    fn from(raw: RawPubsubMessageToSend) -> Self {
        Self::from(&raw)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PubsubMessageRecieved {
    data: Option<String>,
    attributes: Option<HashMap<String, String>>,
    message_id: String,
    publish_time: DateTime<Utc>,
    ordering_key: Option<String>,
}
