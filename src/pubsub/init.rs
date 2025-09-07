use std::time::Duration;

use tokio::time::sleep;

use crate::pubsub::models::ConnectionInfo;

async fn is_live(ctx: &ConnectionInfo) -> bool {
    let result = ctx.client.get(&ctx.host).send().await;
    result.is_ok()
}

pub async fn wait_for_connection(ctx: &ConnectionInfo, timeout: u8) -> bool {
    let mut time_waited = 0;

    while time_waited < timeout {
        let connected = is_live(ctx).await;
        if connected {
            println!("Connection established to: {}", ctx.host);
            return true;
        } else {
            eprintln!("Poll failed, waited {}s so far", time_waited);
            sleep(Duration::from_secs(1)).await;
            time_waited += 1;
        }
    }

    eprintln!("Failed to connect, waited {timeout} seconds");
    false
}
