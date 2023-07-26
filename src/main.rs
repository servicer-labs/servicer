use clap::{Parser, Subcommand};

/// stabled process manager
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Turn an app into a service and start it
    #[command(arg_required_else_help = true)]
    Start {
        /// The file path
        path: String,

        /// Optional custom name for the service
        #[arg(short, long)]
        name: Option<String>
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Start { path , name} => {
            println!("Running {}, {:?}", path, name);

        },
    }
    println!("ok");
}
