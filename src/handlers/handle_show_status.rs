use crate::{
    utils::service_names::{get_short_service_name, is_full_name},
    utils::{
        process_status::{get_memory_usage, get_page_size},
        systemd::{get_active_state, get_main_pid, get_unit_file_state},
    },
};
use bytesize::ByteSize;
use cli_table::{Table, WithTitle};
use psutil::process::Process;
use std::{path::Path, sync::Arc};
use tokio::{fs, sync::Mutex};
use zbus::{
    export::futures_util::future::try_join_all,
    Connection,
};

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
pub async fn handle_show_status() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Arc::new(Mutex::new(Connection::system().await?));
    let page_size = get_page_size().await;
    let stabled_service_names = get_stabled_services().await.unwrap();

    let tasks = stabled_service_names.into_iter().map(|name| {
        let connection = connection.clone();
        tokio::spawn(async move {
            let connection = connection.lock().await;

            let active = get_active_state(&connection, &name).await == "active";
            let enabled_on_boot = get_unit_file_state(&connection, &name).await == "enabled";

            // PID, CPU and memory is 0 for inactive processes
            let (pid, cpu, memory) = if active {
                let pid = get_main_pid(&connection, &name).await.unwrap();

                let memory = get_memory_usage(pid, page_size as u64).await;

                (pid, 0f32, ByteSize(memory).to_string())
            } else {
                (0, 0f32, "0".to_string())
            };

            ServiceStatus {
                pid,
                name: get_short_service_name(&name),
                active,
                enabled_on_boot,
                cpu,
                memory,
            }
        })
    });
    let service_statuses = try_join_all(tasks).await.unwrap();

    cli_table::print_stdout(service_statuses.with_title())?;

    Ok(())
}

/// Get systemd services having an extension `.stabled.service`. We only monitor services created by this tool
async fn get_stabled_services() -> Result<Vec<String>, std::io::Error> {
    let folder_path = "/etc/systemd/system/";

    let folder_path = Path::new(folder_path);

    let mut files = Vec::<String>::new();
    let mut dir = fs::read_dir(folder_path).await.unwrap();

    while let Some(entry) = dir.next_entry().await.unwrap() {
        let path = entry.path();

        if path.is_file() {
            let name = path.file_name().unwrap().to_str().unwrap();
            if is_full_name(name) {
                files.push(name.to_string());
            }
        }
    }

    Ok(files)
}
