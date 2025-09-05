use std::time::Duration;

use reqwest::Client;
use tokio::time::sleep;

use crate::{Config, parser::TopicInitEntry, pubsub::models::Topic};

pub async fn is_live(host: &str, client: &Client) -> bool {
    let result = client.get(host).send().await;
    result.is_ok()
}

pub async fn wait_for_connection(host: &str, client: &Client, timeout: u8) -> bool {
    let mut time_waited = 0;

    while time_waited < timeout {
        let result = is_live(host, client).await;
        match result {
            true => return true,
            false => {
                eprintln!("Poll failed, waited {}s so far", time_waited);
                sleep(Duration::from_secs(1)).await;
                time_waited += 1;
            }
        }
    }

    eprintln!("Connection timed out. Waited {} seconds.", timeout);
    false
}
