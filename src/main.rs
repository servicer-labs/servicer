use clap::{Parser, Subcommand};

mod handlers;
mod utils;

use crate::handlers::handle_create_service::handle_create_service;
use crate::handlers::handle_show_status::handle_show_status;
use crate::handlers::handle_start_service::handle_start_service;
use crate::handlers::handle_stop_service::handle_stop_service;

pub const TOOL_NAME: &str = "stabled";

/// stabled process manager
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create a systemd service for a file at the given path
    #[command(arg_required_else_help = true)]
    Create {
        /// The file path
        path: String,

        /// Optional custom name for the service
        #[arg(short, long)]
        name: Option<String>,

        /// Optional custom interpreter. Input can be the executable's name, eg `python3` or the full path
        /// `usr/bin/python3`. If no input is provided stabled will use the file extension to detect the interpreter.
        #[arg(short, long)]
        interpreter: Option<String>,
    },

    /// Start a service
    #[command(arg_required_else_help = true)]
    Start {
        /// The service name in short form (hello-world) or long form (hello-world.stabled.service)
        name: String,

        /// Enable the service to start at boot. Equivalent to `systemctl enable`. Can enable a running service.
        #[arg(short, long)]
        enable_on_boot: bool,
    },
    // TODO separate enable command
    /// Stop a service
    #[command(arg_required_else_help = true)]
    Stop {
        /// The service name in short form (hello-world) or long form (hello-world.stabled.service).
        name: String,
    },
    /// View the status of your services
    #[command()]
    Status {},
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        Commands::Create {
            path,
            name,
            interpreter,
        } => handle_create_service(path, name, interpreter)
            .await
            .unwrap(),

        Commands::Start {
            name,
            enable_on_boot,
        } => handle_start_service(name, enable_on_boot).await.unwrap(),

        Commands::Stop { name } => handle_stop_service(name).await.unwrap(),

        Commands::Status {} => handle_show_status().await.unwrap(),
    }

    Ok(())
}
