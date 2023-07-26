use clap::{Parser, Subcommand};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

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
        name: Option<String>,

        /// Optional custom interpreter. By default `node` is used for .js and `python3` for .py
        #[arg(short, long)]
        interpreter: Option<String>,
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Start { path: path_as_string , name: custom_name, interpreter: custom_interpreter} => {
            println!("Running {}, {:?}", path_as_string, custom_name);

            // Step 1. Check whether the file exists at path
            let path = Path::new(&path_as_string);
            if !path.exists() {
                panic!("Could not find file at path {}", path.to_str().unwrap_or_default());
            }

            if !path.is_file() {
                panic!("A non-file entity (e.g., directory) exists at the path {}", path.to_str().unwrap_or_default());
            }

            // The file name including extension
            let file_name = path
                .file_name().expect("Failed to get file name")
                .to_str().expect("Failed to stringify file name");

            println!("File name: {}", file_name);

            // Step 2. Get interpreter

            let interpreter = match custom_interpreter {
                Some(_) => custom_interpreter,
                None => get_interpreter(path.extension())
            };
            println!("Got interpreter {:?}", interpreter);

            // Step 3. Create service file
        },
    }
    println!("ok");
}

/// Find the interpreter needed to execute a file with the given extension
///
/// # Arguments
///
/// * `extension`: The file extension
///
fn get_interpreter(extension: Option<&OsStr>) -> Option<String> {
    match extension {
        Some(extension_os_str) => {
            let extension_str = extension_os_str.to_str().expect("failed to stringify extension");

            let interpreter = match extension_str {
                "js" => "node",
                "py" => "python3",
                _ => panic!("No interpeter found for extension {}. Please provide a custom interpeter and try again.", extension_str)
            };

            Some(interpreter.to_string())
        },
        None => None
    }
}

fn create_service_file(name: &str, interpreter: Option<String>) {

}