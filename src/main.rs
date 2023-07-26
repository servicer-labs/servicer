use clap::{Parser, Subcommand};
use indoc::formatdoc;
use which::which;
use std::ffi::OsStr;
use std::io::Write;
use std::{fs, env};
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
    /// Turn an app into a service and start it. Must be called in `sudo` mode
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

    // TODO exit if systemd is not installed

    // TODO exit if not linux

    match args.command {
        Commands::Start {
            path: path_as_string ,
            name: custom_name,
            interpreter: custom_interpreter
        } => {
            println!("Running {}, {:?}", path_as_string, custom_name);

            // Step 0. Check if service already exists

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
                .to_str().expect("Failed to stringify file name").to_string();

            println!("File name: {}", file_name);

            // Step 2. Get interpreter

            let interpreter = match custom_interpreter {
                Some(_) => custom_interpreter,
                None => get_interpreter(path.extension())
            };
            println!("Got interpreter {:?}", interpreter);

            // Step 3. Create service file
            let service_name = custom_name.unwrap_or(file_name.to_string());

            let working_directory = fs::canonicalize(path.parent().unwrap()).unwrap()
                .to_str().unwrap().to_string();

            println!("got working directory {:?}", working_directory);

            create_service_file(&service_name, &working_directory, interpreter, &file_name).unwrap();
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

/// Creates a systemd service file at `/etc/systemd/system/{}.stabled.service`
///
/// # Arguments
///
/// * `service_name`- Name of the service
/// * `working_directory` - Working directory of the file to execute
/// * `interpreter` - The executable used to run the app, eg. `node` or `python3`. The executable
/// must be visible from path for a sudo user. Note that the app itself does not run in sudo.
/// TODO allow users to pass the interpreter path.
/// * `file_name` - Name of the file to run
///
fn create_service_file(
    service_name: &String,
    working_directory: &String,
    interpreter: Option<String>,
    file_name: &String
) -> std::io::Result<()> {
    const TOOL_NAME: &str = "stabled";

    // This gets `root` instead of `hp` if sudo is used
    let user = env::var("SUDO_USER").expect("Must be in sudo mode. ENV variable $SUDO_USER not found");
    let exec_start = match interpreter {
        Some(interpreter) => {
            // Find full path of interpreter
            // caveat- since this function is called in sudo mode, `node` and `python` paths must be
            // readable in sudo. python3 works out of the box but nvm requires a hack.
            let interpreter_path = which(&interpreter)
                .expect(&format!("Could not find executable for {}", interpreter))
                .to_str().expect(&format!("Failed to stringify interpreter path for {}.", interpreter))
                .to_string();

            format!("{} {}", interpreter_path, file_name)
        },
        None => file_name.clone()
    };

    // Replacement for format!(). This proc macro removes spaces produced by indentation.
    let service_body = formatdoc! {
        r#"
        # This file was generated by {}. Do not edit unless you know what you are doing.
        [Unit]
        Description={}: {}
        After=network.target

        [Service]
        Type=simple
        User={}

        WorkingDirectory={}
        ExecStart={}

        [Install]
        WantedBy=multi-user.target
        "#,
        TOOL_NAME,
        TOOL_NAME,
        service_name,
        user,
        working_directory,
        exec_start
    };

    println!("{}", service_body);

    let service_file_path = format!("/etc/systemd/system/{}.{}.service", service_name, TOOL_NAME);

    if Path::new(&service_file_path).exists() {
        panic!("Service {} is already deployed at {}", service_name, service_file_path);
    }

    // Create the service file and write the content
    let mut file = fs::File::create(service_file_path)?;
    file.write_all(service_body.as_bytes())?;

    Ok(())

}
