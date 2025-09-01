use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: PubsubCommands,

    #[arg(short, long)]
    pub project_id: String,

    #[arg(long, env)]
    pub host: String,
}

#[derive(Subcommand)]
pub enum PubsubCommands {
    #[command(subcommand)]
    Topics(TopicCommands),
}

#[derive(Subcommand)]
pub enum TopicCommands {
    Create { name: String },
    List,
    Info,
    Delete { name: String },
}
