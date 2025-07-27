use reqwest::Client;

use super::models::{ListTopicsResponse, Topic};

pub async fn list(
    project_id: &str,
    client: Client,
    address: &str,
) -> Result<ListTopicsResponse, reqwest::Error> {
    let endpoint = format!("{address}/v1/projects/{project_id}/topics");
    println!("Making request at: {endpoint}");
    let response = client.get(endpoint).send().await?;
    let payload = response.json().await?;

    Ok(payload)
}
