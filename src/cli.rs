use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: PubsubCommands,
}

#[derive(Subcommand)]
pub enum PubsubCommands {
    #[command(subcommand)]
    Topics(TopicCommands),
}

#[derive(Subcommand)]
pub enum TopicCommands {
    Create(CreateTopicArgs),
}

#[derive(Args)]
pub struct CreateTopicArgs {
    pub name: String,
}
