use std::sync::Arc;

use clap::Parser;

use cli::Cli;
use reqwest::Client;

mod cli;
mod pubsub;

use crate::cli::{PubsubCommands, TopicCommands};

struct Config {
    client: Arc<Client>,
    project_id: String,
    host: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = Client::new();
    let config = Config {
        client: Arc::new(client),
        project_id: cli.project_id.to_string(),
        host: cli.host.to_string(),
    };

    let result = match &cli.commands {
        PubsubCommands::Topics(cmd) => handle_topics(cmd, &config).await,
    };

    match result {
        Ok(_) => println!("Completed Successfully"),
        Err(e) => eprintln!("Error occured: {}", e),
    }
}

async fn handle_topics(cmd: &TopicCommands, config: &Config) -> Result<(), reqwest::Error> {
    match cmd {
        TopicCommands::Create { name } => {
            println!("Creating topic {name} on project {}", config.project_id);
            let result =
                pubsub::topics::create(&config.project_id, &config.client, &config.host, name)
                    .await?;

            println!("Topic has been created {:?}", result);
            Ok(())
        }
        TopicCommands::List => {
            let topic_list =
                pubsub::topics::list(&config.project_id, &config.client, &config.host).await?;

            println!("Topics Retreived:");
            topic_list.topics.iter().for_each(|t| println!("{:?}", t));
            Ok(())
        }
        TopicCommands::Info => todo!(),
        TopicCommands::Delete { name } => {
            println!("Delete topic {name} from project {}", config.project_id);
            pubsub::topics::delete(&config.client, &config.host, name).await?;
            println!("Topic deleted");
            Ok(())
        }
    }
}
