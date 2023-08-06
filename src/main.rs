use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod handlers;
mod utils;

use crate::handlers::handle_create_service::handle_create_service;
use crate::handlers::handle_disable_service::handle_disable_service;
use crate::handlers::handle_enable_service::handle_enable_service;
use crate::handlers::handle_show_logs::handle_show_logs;
use crate::handlers::handle_show_status::handle_show_status;
use crate::handlers::handle_start_service::handle_start_service;
use crate::handlers::handle_stop_service::handle_stop_service;

/// servicer process manager
#[derive(Parser, Debug)]
#[command(author, about, version)]
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
        path: PathBuf,

        /// Optional custom name for the service
        #[arg(short, long)]
        name: Option<String>,

        /// Start the service
        #[arg(short, long)]
        start: bool,

        /// Enable the service to start every time on boot. This doesn't immediately start the service, to do that run
        /// together with `start
        #[arg(short, long)]
        enable: bool,

        /// Auto-restart on failure. Default false. You should edit the .service file for more advanced features.
        /// The service must be enabled for auto-restart to work.
        #[arg(short = 'r', long)]
        auto_restart: bool,

        /// Optional custom interpreter. Input can be the executable's name, eg `python3` or the full path
        /// `usr/bin/python3`. If no input is provided servicer will use the file extension to detect the interpreter.
        #[arg(short, long)]
        interpreter: Option<String>,

        /// Optional environment variables. To run `FOO=BAR node index.js` call `ser create index.js --env_vars "FOO=BAR"`
        #[arg(short = 'v', long)]
        env_vars: Option<String>,

        /// Optional args passed to the file. Eg. to run `node index.js --foo bar` call `ser create index.js -- --foo bar`
        #[arg(last = true)]
        internal_args: Vec<String>,
    },

    /// Start a service
    #[command(arg_required_else_help = true)]
    Start {
        /// The service name, eg. hello-world
        name: String,
    },
    /// Stop a service
    #[command(arg_required_else_help = true)]
    Stop {
        /// The service name, eg. hello-world
        name: String,
    },

    /// Enable a service to start on boot. Doesn't immediately start the service. To do so use the `start` command.
    #[command(arg_required_else_help = true)]
    Enable {
        /// The service name, eg. hello-world
        name: String,
    },

    /// Disable a service from starting on boot
    #[command(arg_required_else_help = true)]
    Disable {
        /// The service name, eg. hello-world
        name: String,
    },

    /// View the status of your services
    #[command()]
    Status {},

    /// View logs for a service
    #[command()]
    Logs {
        /// The service name
        name: String,

        /// Output the last N lines, instead of the default 15
        #[arg(short = 'n', long, default_value_t = 15)]
        lines: u32,

        /// Follow the logs as they change
        #[arg(short, long, default_value_t = false)]
        follow: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        Commands::Create {
            path,
            name,
            start,
            enable,
            auto_restart,
            interpreter,
            env_vars,
            internal_args,
        } => handle_create_service(
            path,
            name,
            start,
            enable,
            auto_restart,
            interpreter,
            env_vars,
            internal_args,
        )
        .await
        .unwrap(),

        Commands::Start { name } => handle_start_service(name, true).await.unwrap(),

        Commands::Stop { name } => handle_stop_service(name).await.unwrap(),

        Commands::Enable { name } => handle_enable_service(name, true).await.unwrap(),

        Commands::Disable { name } => handle_disable_service(name).await.unwrap(),

        Commands::Status {} => handle_show_status().await.unwrap(),

        Commands::Logs {
            name,
            lines,
            follow,
        } => handle_show_logs(name, lines, follow).await.unwrap(),
    }

    Ok(())
}
