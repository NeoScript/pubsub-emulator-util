use std::{process::exit, sync::Arc};

use clap::Parser;

use cli::Cli;
use reqwest::Client;

mod cli;
mod parser;
mod pubsub;

use crate::{
    cli::{InitArgs, PubsubCommands, TopicCommands},
    pubsub::{init::wait_for_connection, models::Topic},
};

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
        project_id: "my-project".to_string(), // TODO: fix this again
        host: "http://localhost:8090".to_string(), //TODO: fix this too
    };

    let result = match &cli.commands {
        PubsubCommands::Topics(cmd) => handle_topics(cmd, &config).await,
        PubsubCommands::Init(init_args) => handle_init(init_args, &config).await,
    };

    if let Err(e) = result {
        eprintln!("Error occured: {}", e)
    }
}

async fn handle_init(args: &InitArgs, config: &Config) -> Result<(), reqwest::Error> {
    let init_config = parser::parse_init_file(&args.file);

    let found_conn = wait_for_connection(&args.host, &config.client, args.timeout).await;
    match found_conn {
        true => {
            println!("Connection established!")
        }
        false => {
            eprintln!("Timed out");
            exit(1);
        }
    }

    // Create all the topics
    if let Ok(init) = init_config {
        println!("initialization file parsed: {}", args.file);
        let project_id = init.project_id;
        println!("using project: {project_id}");

        for topic in init.topics {
            let result = pubsub::topics::create(
                &project_id,
                &config.client,
                &config.host,
                &topic.clone().into(),
            )
            .await;

            match result {
                Ok(t) => {
                    println!("Created topic: {:?}", t);
                }
                Err(e) => eprintln!("Error creating topic {:?} -> {}", topic.name, e),
            };

            // TODO: make the pull subscriptions
            // TODO: now make the push subscriptions
        }
    } else {
        eprintln!("Error parsing init file")
    }
    Ok(())
}

async fn handle_topics(cmd: &TopicCommands, config: &Config) -> Result<(), reqwest::Error> {
    match cmd {
        TopicCommands::Create { name } => {
            let topic = Topic {
                name: name.to_string(),
                labels: None,
            };
            println!("Creating topic {topic:?} on project {}", config.project_id);
            let result =
                pubsub::topics::create(&config.project_id, &config.client, &config.host, &topic)
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
