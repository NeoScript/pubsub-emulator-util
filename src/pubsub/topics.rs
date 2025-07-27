use reqwest::Client;

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
    topic_id: &str,
) -> Result<Topic, reqwest::Error> {
    let endpoint = format!("{address}/v1/projects/{project_id}/topics/{topic_id}");

    // NOTE: for some reason we have to end this empty '{}' value
    let payload = "{}";
    let response = client.put(endpoint).body(payload).send().await?;

    let created_topic = response.json().await?;
    Ok(created_topic)
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
