use crate::{TOOL_NAME, systemd::{get_unit_file_state, get_active_state, get_main_pid}};
use std::{fs, path::Path};
use cli_table::{format::Justify, Table, WithTitle};

#[derive(Table)]
struct ServiceStatus {
    /// Process ID
    pid: u32,

    /// The short service name, excluding '.stabled.service'
    name: String,

    /// Active state
    active: bool,

    // /// Load the service on boot
    enabled_on_boot: bool,

    // /// CPU usage. Formatted string in %
    // cpu: String,

    // /// RAM usage. Formatted string with MB, KB and other units
    // mem: String
}

pub fn handle_show_status() -> Result<(), Box<dyn std::error::Error>> {
    let connection = zbus::blocking::Connection::system().unwrap();
    let stabled_service_names = get_stabled_services();

    let service_statuses: Vec<ServiceStatus> = stabled_service_names.into_iter()
        .map(|name| {
            let active = get_active_state(&connection, &name) == "active";
            let enabled_on_boot = get_unit_file_state(&connection, &name) == "enabled";
            let (pid) = if active {
                (get_main_pid(&connection, &name).unwrap())
            } else {
                (0)
            };
            ServiceStatus {
                pid,
                name,
                active,
                enabled_on_boot,
            }
        })
        .collect();

    cli_table::print_stdout(service_statuses.with_title())?;

    Ok(())
}

fn get_stabled_services() -> Vec<String> {
    let folder_path = "/etc/systemd/system/";
    let file_extension = format!(".{TOOL_NAME}.service");

    let folder_path = Path::new(folder_path);

    // Read the contents of the directory
    if let Ok(entries) = fs::read_dir(folder_path) {
        let stabled_services: Vec<String> = entries
            // Filter files only
            .filter_map(|entry| {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    if let Some(file_name) = file_name.to_str() {
                        // Check if the file has the desired extension
                        if file_name.ends_with(&file_extension) {
                            return Some(file_name.to_string());
                        }
                    }
                }
                None
            })
            .collect();

        return stabled_services;
    }

    // Return an empty vector if there was an error reading the directory
    Vec::new()
}

fn print_table() {
}
