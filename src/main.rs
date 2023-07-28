use clap::{Parser, Subcommand};

mod handlers;
mod systemd;

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
    /// Daemonize an app at a given path or start an existing service
    #[command(arg_required_else_help = true)]
    Start {
        /// The file path or service to start
        path_or_service: String,

        /// Optional custom name for the daemon
        #[arg(short, long)]
        name: Option<String>,

        /// Optional custom interpreter. By default `node` is used for .js and `python3` for .py
        #[arg(short, long)]
        interpreter: Option<String>,

        #[arg(short, long, default_value_t = false)]
        force: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // TODO exit if systemd is not installed

    // TODO exit if not linux

    match args.command {
        Commands::Start {
            path_or_service,
            name: custom_name,
            interpreter: custom_interpreter,
            force,
        } => handle_start(path_or_service, custom_name, custom_interpreter, force).unwrap(),
    }
    println!("ok");

    Ok(())
}
