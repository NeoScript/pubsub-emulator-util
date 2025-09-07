use reqwest::{Client, StatusCode};

use crate::pubsub::models::{
    ConnectionInfo, PubsubMessageRecieved, RawPubsubMessageToSend, SendablePubsubMessage,
};

use super::models::{ListTopicsResponse, Topic};

pub async fn list(ctx: &ConnectionInfo) -> Result<ListTopicsResponse, reqwest::Error> {
    let endpoint = format!("{}/v1/projects/{}/topics", ctx.host, ctx.project_id);
    println!("Making request at: {endpoint}");
    let response = ctx.client.get(endpoint).send().await?;
    let payload = response.json().await?;

    Ok(payload)
}

pub async fn create(ctx: &ConnectionInfo, topic: &Topic) -> Result<Topic, reqwest::Error> {
    let topic_path = topic.full_path(&ctx.project_id);
    let endpoint = format!("{}/v1/{}", ctx.host, topic_path);
    let response = ctx.client.put(endpoint).send().await?;

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
pub async fn delete(ctx: &ConnectionInfo, topic: &Topic) -> Result<(), reqwest::Error> {
    let topic_path = topic.full_path(&ctx.project_id);
    let endpoint = format!("{}/v1/{}", ctx.host, topic_path);

    let response = ctx.client.delete(endpoint).send().await?;
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
