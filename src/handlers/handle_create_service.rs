use std::{env, fs, io::Write, path::Path};
use which::which;

use indoc::formatdoc;

use crate::{utils::service_names::get_full_service_name, TOOL_NAME};

/// Creates a new systemd service file.
///
/// # Arguments
///
/// * `path` - Create service for a file at this path
/// * `custom_name`
/// * `custom_interpreter`
///
pub fn handle_create_service(
    path: String,
    custom_name: Option<String>,
    custom_interpreter: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new(&path);
    if !file_path.exists() {
        panic!(
            "Could not find file at path {}",
            file_path.to_str().unwrap_or_default()
        );
    }

    if !file_path.is_file() {
        panic!(
            "A non-file entity (e.g., directory) exists at the path {}",
            file_path.to_str().unwrap_or_default()
        );
    }

    // The file name including extension, eg. index.js
    let file_name = file_path
        .file_name()
        .expect("Failed to get file name")
        .to_str()
        .expect("Failed to stringify file name")
        .to_string();

    let service_name = custom_name.unwrap_or(file_name.to_string());
    let full_service_name = get_full_service_name(&service_name);

    // Create file if it doesn't exist
    let service_file_path = format!("/etc/systemd/system/{}", full_service_name.clone());

    if Path::new(&service_file_path).exists() {
        panic!("Service {service_name} already exists at {service_file_path}. Provide a custom name with --name or delete the existing service with `{TOOL_NAME} delete {service_name}");
    } else {
        let interpreter = match custom_interpreter {
            Some(_) => custom_interpreter,
            None => get_interpreter(file_path.extension()),
        };

        let working_directory = fs::canonicalize(file_path.parent().unwrap())
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        create_service_file(
            &service_name,
            &service_file_path,
            &working_directory,
            interpreter,
            &file_name,
        )
        .unwrap();

        println!("Service {service_name} created at {service_file_path}. To start run `{TOOL_NAME} start {service_name}`");
    }

    Ok(())
}

/// Find the interpreter needed to execute a file with the given extension
///
/// # Arguments
///
/// * `extension`: The file extension
///
fn get_interpreter(extension: Option<&std::ffi::OsStr>) -> Option<String> {
    match extension {
        Some(extension_os_str) => {
            let extension_str = extension_os_str
                .to_str()
                .expect("failed to stringify extension");

            let interpreter = match extension_str {
              "js" => "node",
              "py" => "python3",
              _ => panic!("No interpeter found for extension {}. Please provide a custom interpeter and try again.", extension_str)
          };

            Some(interpreter.to_string())
        }
        None => None,
    }
}

/// Creates a systemd service file at `/etc/systemd/system/{}.stabled.service` and returns the unit name
///
/// # Arguments
///
/// * `service_name`- Name of the service without '.stabled.service' in the end
/// * `service_file_path` - Path where the service file will be written
/// * `working_directory` - Working directory of the file to execute
/// * `interpreter` - The executable used to run the app, eg. `node` or `python3`. The executable
/// must be visible from path for a sudo user. Note that the app itself does not run in sudo.
/// TODO allow users to pass the interpreter path.
/// * `file_name` - Name of the file to run
///
fn create_service_file(
    service_name: &String,
    service_file_path: &String,
    working_directory: &String,
    interpreter: Option<String>,
    file_name: &String,
) -> std::io::Result<()> {
    // This gets `root` instead of `hp` if sudo is used
    let user =
        env::var("SUDO_USER").expect("Must be in sudo mode. ENV variable $SUDO_USER not found");
    let exec_start = match interpreter {
        Some(interpreter) => {
            // Find full path of interpreter
            // caveat- since this function is called in sudo mode, `node` and `python` paths must be
            // readable in sudo. python3 works out of the box but nvm requires a hack.
            let interpreter_path = which(&interpreter)
                .expect(&format!("Could not find executable for {}", interpreter))
                .to_str()
                .expect(&format!(
                    "Failed to stringify interpreter path for {}.",
                    interpreter
                ))
                .to_string();

            format!("{} {}", interpreter_path, file_name)
        }
        None => file_name.clone(),
    };

    // Replacement for format!(). This proc macro removes spaces produced by indentation.
    let service_body = formatdoc! {
        r#"
      # This file was generated by {TOOL_NAME}. Do not edit unless you know what you are doing.
      [Unit]
      Description={TOOL_NAME}: {service_name}
      After=network.target

      [Service]
      Type=simple
      User={user}

      WorkingDirectory={working_directory}
      ExecStart={exec_start}

      [Install]
      WantedBy=multi-user.target
      "#
    };

    // Create the service file and write the content
    let mut file = fs::File::create(service_file_path)?;
    file.write_all(service_body.as_bytes())?;

    // TODO show status

    Ok(())
}
