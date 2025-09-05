use std::path::Path;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: PubsubCommands,
    // #[arg(long, env)]
    // pub host: String,
}

#[derive(Subcommand)]
pub enum PubsubCommands {
    #[command(subcommand)]
    Topics(TopicCommands),

    #[command()]
    Init(InitArgs),
}

#[derive(Parser, Debug)]
pub struct InitArgs {
    #[arg(
        short,
        long,
        help = "path to file containing initial state",
        value_name = "FILE_PATH"
    )]
    pub file: String,

    #[arg(
        long,
        env("PUBSUB_EMULATOR_HOST"),
        help = "the pubsub emulator host in the following format: `http://addr:port`"
    )]
    pub host: String,

    #[arg(
        short,
        long,
        value_name = "SECONDS",
        value_parser = clap::value_parser!(u8).range(0..=255),
        help = "how long (in seconds) to poll the pubsub emulator host before giving up."
    )]
    pub timeout: Option<u8>,
}

#[derive(Subcommand)]
pub enum TopicCommands {
    Create { name: String },
    List,
    Info,
    Delete { name: String },
}
