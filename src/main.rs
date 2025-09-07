use std::{process::exit, sync::Arc};

use clap::Parser;

use cli::Cli;
use reqwest::Client;

mod cli;
mod parser;
mod pubsub;

use crate::{
    cli::{InitArgs, PubsubCommands, TopicCommands},
    pubsub::init::wait_for_connection,
    pubsub::models::{ConnectionInfo, Topic},
};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = Client::new();
    let config = ConnectionInfo {
        client: Arc::new(client),
        project_id: "my-project".to_string(), // TODO: fix this again
        host: cli.host,
    };

    let result = match &cli.commands {
        PubsubCommands::Topics(cmd) => handle_topic_commands(cmd, &config).await,
        PubsubCommands::Init(init_args) => handle_init(init_args, &config).await,
    };

    if let Err(e) = result {
        eprintln!("Error occured: {}", e)
    }
}

async fn handle_init(args: &InitArgs, ctx: &ConnectionInfo) -> Result<(), reqwest::Error> {
    let init_config = parser::parse_init_file(&args.file).unwrap_or_else(|e| {
        eprintln!("Error parsing init file: {}", e);
        exit(1);
    });

    println!("Initialization Configuration accepted:");
    let init_json = serde_json::to_string(&init_config).expect("should convert to json");
    println!("{init_json}");
    println!();

    // Define a new ctx to use the provided host from args and project_id from file
    let ctx = &ConnectionInfo {
        project_id: init_config.project_id,
        ..ctx.clone()
    };

    let connection_established = wait_for_connection(ctx, args.timeout).await;
    if !connection_established {
        eprintln!("Timed out whilst waiting for connection");
        exit(1)
    }

    for topic in init_config.topics {
        let result = pubsub::topics::create(ctx, &topic.clone().into()).await;

        match result {
            Ok(t) => println!("{:?} created successfully", t.name),
            Err(e) => eprintln!("Error creating topic {:?} -> {}", topic.name, e),
        };

        // TODO: make the pull subscriptions
        // TODO: now make the push subscriptions
    }
    Ok(())
}

async fn handle_topic_commands(
    cmd: &TopicCommands,
    ctx: &ConnectionInfo,
) -> Result<(), reqwest::Error> {
    match cmd {
        TopicCommands::Create { name } => {
            let topic = Topic {
                name: name.to_string(),
                labels: None,
            };
            pubsub::topics::create(ctx, &topic).await?;
            Ok(())
        }
        TopicCommands::List => {
            let topic_list = pubsub::topics::list(ctx).await?;
            topic_list.topics.iter().for_each(|t| println!("{:?}", t));
            Ok(())
        }
        TopicCommands::Info => todo!(),
        TopicCommands::Delete { name } => {
            let topic_to_delete = Topic {
                name: name.to_string(),
                labels: None,
            };
            pubsub::topics::delete(ctx, &topic_to_delete).await?;
            Ok(())
        }
    }
}
