use base64::{Engine as _, engine::general_purpose::STANDARD};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use std::{collections::HashMap, str::Bytes};

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

impl From<RawPubsubMessageToSend> for SendablePubsubMessage {
    fn from(raw: RawPubsubMessageToSend) -> Self {
        Self {
            data: raw.data.map(|payload| STANDARD.encode(payload)),
            attributes: raw.attributes,
            ordering_key: raw.ordering_key,
        }
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
