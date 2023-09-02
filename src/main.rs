use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod handlers;
mod utils;

use handlers::handle_create_service::handle_create_service;
use handlers::handle_delete_service::handle_delete_service;
use handlers::handle_disable_service::handle_disable_service;
use handlers::handle_edit_service_file::handle_edit_service_file;
use handlers::handle_enable_service::handle_enable_service;
use handlers::handle_print_paths::handle_print_paths;
use handlers::handle_print_service_file::handle_print_service_file;
use handlers::handle_reload_service::handle_reload_service;
use handlers::handle_show_logs::handle_show_logs;
use handlers::handle_show_status::handle_show_status;
use handlers::handle_start_service::handle_start_service;
use handlers::handle_stop_service::handle_stop_service;

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

    /// Open a text editor to create or edit the .service file for a service
    #[command(arg_required_else_help = true)]
    Edit {
        /// The service name, eg. hello-world
        name: String,

        /// Custom editor to use. Default nano
        #[arg(short, long, default_value = "nano")]
        editor: String,
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

    /// Delete a service, stopping and disabling it if necessary and removing the .service file (alias: rm)
    #[command(arg_required_else_help = true, alias = "rm")]
    Delete {
        /// The service name, eg. hello-world
        name: String,
    },

    /// View the status of your services (alias: ls)
    #[command(alias = "ls")]
    Status {},

    /// View logs for a service
    #[command(arg_required_else_help = true)]
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

    /// Reloads a service having an `ExecScript`
    #[command(arg_required_else_help = true)]
    Reload {
        /// The service name
        name: String,
    },

    /// Display contents of the .service file of a service
    #[command(arg_required_else_help = true)]
    Cat {
        /// The service name, eg hello-world
        name: String,
    },

    /// Print the of the .service file and unit file
    #[command(arg_required_else_help = true)]
    Which {
        /// The service name, eg hello-world
        name: String,
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
        } => {
            handle_create_service(
                path,
                name,
                start,
                enable,
                auto_restart,
                interpreter,
                env_vars,
                internal_args,
            )
            .await?
        }

        Commands::Start { name } => handle_start_service(name, true).await?,

        Commands::Stop { name } => handle_stop_service(name, true).await?,

        Commands::Enable { name } => handle_enable_service(name, true).await?,

        Commands::Disable { name } => handle_disable_service(name, true).await?,

        Commands::Status {} => handle_show_status().await?,

        Commands::Logs {
            name,
            lines,
            follow,
        } => handle_show_logs(name, lines, follow).await?,

        Commands::Edit { name, editor } => handle_edit_service_file(name, editor).await?,

        Commands::Reload { name } => handle_reload_service(name, true).await?,

        Commands::Cat { name } => handle_print_service_file(name).await?,

        Commands::Which { name } => handle_print_paths(name).await?,

        Commands::Delete { name } => handle_delete_service(name).await?,
    }

    Ok(())
}
