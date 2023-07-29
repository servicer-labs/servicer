use clap::{Parser, Subcommand};

mod handlers;
mod systemd;

use crate::handlers::handle_create_service::handle_create_service;
use crate::handlers::handle_start::handle_start;

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

    /// Create a systemd service for an app at a file path. If the input is a service name,
    /// try to start the service.
    #[command(arg_required_else_help = true)]
    Start {
        /// The file path or name of the service to start
        path_or_service: String,

        /// Optional custom name for the service
        #[arg(short, long)]
        name: Option<String>,

        /// Optional custom interpreter. Input can be the executable's name, eg `python3` or the full path
        /// `usr/bin/3`. If no input is provided stabled will use the file extension to detect the interpreter.
        #[arg(short, long)]
        interpreter: Option<String>,

        /// Force start a service, overriding another service with the same name. Default false.
        #[arg(short, long, default_value_t = false)]
        force: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        Commands::Create {
            path,
            name,
            interpreter,
        } => handle_create_service(path, name, interpreter).unwrap(),

        Commands::Start {
            path_or_service,
            name: custom_name,
            interpreter: custom_interpreter,
            force,
        } => handle_start(path_or_service, custom_name, custom_interpreter, force).unwrap(),
    }

    Ok(())
}
