use crate::{
    systemd::{get_active_state, get_main_pid, get_unit_file_state},
    TOOL_NAME,
};
use bytesize::ByteSize;
use cli_table::{Table, WithTitle};
use psutil::process::Process;
use std::{fs, path::Path};

#[derive(Table)]
struct ServiceStatus {
    /// Process ID
    pid: u32,

    /// The short service name, excluding '.stabled.service'
    name: String,

    /// Active state
    active: bool,

    /// Load the service on boot
    #[table(title = "enable on boot")]
    enabled_on_boot: bool,

    /// CPU usage. Formatted string in %
    #[table(title = "cpu %")]
    cpu: f32,

    /// RAM usage. Formatted string with MB, KB and other units
    memory: String,
}

/// Display the status of your services
pub fn handle_show_status() -> Result<(), Box<dyn std::error::Error>> {
    let connection = zbus::blocking::Connection::system().unwrap();
    let stabled_service_names = get_stabled_services();

    // TODO consider async to parallelize reads
    let service_statuses: Vec<ServiceStatus> = stabled_service_names
        .into_iter()
        .map(|name| {
            let active = get_active_state(&connection, &name) == "active";
            let enabled_on_boot = get_unit_file_state(&connection, &name) == "enabled";
            let (pid, cpu, memory) = if active {
                let pid = get_main_pid(&connection, &name).unwrap();

                // cpu_percent() must be called twice to find CPU usage
                // TODO optimize by writing in a non-blocking fashion
                let mut process = Process::new(pid).unwrap();
                process.cpu_percent().unwrap();
                std::thread::sleep(std::time::Duration::from_millis(200));
                let cpu = process.cpu_percent().unwrap();

                let memory_info = process.memory_info().unwrap();

                // Formula used by Ubuntu task manager
                let memory = memory_info.rss() - memory_info.shared();
                (pid, cpu, ByteSize(memory).to_string())
            } else {
                (0, 0f32, "0".to_string())
            };
            ServiceStatus {
                pid,
                name: get_short_service_name(&name).to_string(),
                active,
                enabled_on_boot,
                cpu,
                memory,
            }
        })
        .collect();

    cli_table::print_stdout(service_statuses.with_title())?;

    Ok(())
}

/// Get systemd services having an extension `.stabled.service`. We only monitor services created by this tool
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

/// Shortens the service name from `example.stabled.service` to `example`
///
/// # Arguments
///
/// * `full_service_name`
///
fn get_short_service_name(full_service_name: &str) -> &str {
    let file_extension = format!(".{TOOL_NAME}.service");

    full_service_name.trim_end_matches(file_extension.as_str())
}
