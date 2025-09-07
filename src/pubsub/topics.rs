use reqwest::{Client, StatusCode};

use crate::pubsub::models::{PubsubMessageRecieved, RawPubsubMessageToSend, SendablePubsubMessage};

use super::models::{ListTopicsResponse, Topic};

pub async fn list(
    project_id: &str,
    client: &Client,
    address: &str,
) -> Result<ListTopicsResponse, reqwest::Error> {
    let endpoint = format!("{address}/v1/projects/{project_id}/topics");
    println!("Making request at: {endpoint}");
    let response = client.get(endpoint).send().await?;
    let payload = response.json().await?;

    Ok(payload)
}

pub async fn create(
    project_id: &str,
    client: &Client,
    address: &str,
    topic: &Topic,
) -> Result<Topic, reqwest::Error> {
    let topic_path = topic.full_path(project_id);
    let endpoint = format!("{address}/v1/{topic_path}");
    let response = client.put(endpoint).send().await?;

    match response.status() {
        StatusCode::OK | StatusCode::CREATED => {
            let created_topic = response.json().await?;
            Ok(created_topic)
        }
        StatusCode::CONFLICT => {
            eprintln!("Conflict when creating topic, usually due to a duplicate topic");
            Err(response.error_for_status().err().unwrap())
        }
        _ => Err(response.error_for_status().err().unwrap()),
    }
}

/// NOTE: topic_path should be in format of 'project/{project_id}/topics/{topic_id}'
pub async fn delete(
    client: &Client,
    address: &str,
    topic_path: &str,
) -> Result<(), reqwest::Error> {
    let endpoint = format!("{address}/v1/{topic_path}");

    let response = client.delete(endpoint).send().await?;
    match response.error_for_status() {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Failed with error: {e}");
            Err(e)
        }
    }
}

pub async fn publish(
    client: &Client,
    address: &str,
    topic: &Topic,
    messages: Vec<&RawPubsubMessageToSend>,
) -> Result<Vec<PubsubMessageRecieved>, reqwest::Error> {
    let topic_path = &topic.name;
    let endpoint = format!("{address}/v1/{topic_path}");

    let sendable_payload: Vec<SendablePubsubMessage> = messages
        .iter()
        .map(|m| SendablePubsubMessage::from(*m))
        .collect();

    let response = client.post(endpoint).json(&sendable_payload).send().await?;
    let payload = response.json().await?;

    Ok(payload)
}
