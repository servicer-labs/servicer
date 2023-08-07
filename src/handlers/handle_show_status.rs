use crate::{
    utils::service_names::{get_short_service_name, is_full_name},
    utils::{
        process_status::{get_cpu_time, get_memory_usage, get_page_size},
        systemd::{get_active_state, get_main_pid, get_unit_file_state},
    },
};
use bytesize::ByteSize;
use cli_table::{Table, WithTitle};
use futures;
use std::path::Path;
use tokio::fs;
use zbus::Connection;

#[derive(Table, Clone)]
pub struct ServiceStatus {
    /// Process ID
    pub pid: u32,

    /// The short service name, excluding '.ser.service'
    pub name: String,

    /// Active state
    pub active: String,

    /// Load the service on boot
    #[table(title = "enable on boot")]
    pub enabled_on_boot: bool,

    /// CPU usage. Formatted string in %
    #[table(title = "cpu %")]
    pub cpu: f32,

    /// RAM usage. Formatted string with MB, KB and other units
    pub memory: String,
}

/// Display the status of your services
pub async fn handle_show_status() -> Result<(), Box<dyn std::error::Error>> {
    let page_size = get_page_size().await;
    let services = get_servicer_services().await.unwrap();

    let connection = Connection::system().await?;

    let mut active_process_exists = true;
    let mut service_statuses: Vec<ServiceStatus> = vec![];

    for full_service_name in services {
        let active = get_active_state(&connection, &full_service_name).await;

        let unit_state = get_unit_file_state(&connection, &full_service_name).await;
        let enabled_on_boot = unit_state == "enabled" || unit_state == "enabled-runtime";

        // PID, CPU and memory is 0 for inactive and errored processes
        let (pid, cpu, memory) = if active == "active" {
            active_process_exists = true;

            let pid = get_main_pid(&connection, &full_service_name).await.unwrap();
            let memory = get_memory_usage(pid, page_size as u64).await;

            (pid, 0f32, ByteSize(memory).to_string())
        } else {
            (0, 0f32, "0".to_string())
        };

        service_statuses.push(ServiceStatus {
            pid,
            name: get_short_service_name(&full_service_name),
            active,
            enabled_on_boot,
            cpu,
            memory,
        });
    }

    // CPU time algorithm- Find the change in CPU time over an interval, then divide by the interval
    // Source- https://github.com/dalance/procs/blob/ba703e98cd44be46ba32e084f1474d81b9a7f660/src/columns/usage_cpu.rs#L36C57-L36C83

    // Sleep duration in ms
    const SLEEP_DURATION: u32 = 100;

    if active_process_exists {
        // We only need to sleep once with this method
        let initial_cpu_times = get_cpu_times(service_statuses.clone()).await;
        tokio::time::sleep(tokio::time::Duration::from_millis(SLEEP_DURATION as u64)).await;
        let final_cpu_times = get_cpu_times(service_statuses.clone()).await;

        let tps = clock_ticks_per_second();

        for i in 0..service_statuses.len() {
            let initial_time = initial_cpu_times.get(i).unwrap().clone();
            let final_time = final_cpu_times.get(i).unwrap().clone();
            let usage_ms = (final_time - initial_time) * 1000 / tps;
            let cpu_usage = usage_ms as f32 * 100.0 / SLEEP_DURATION as f32;

            let status = service_statuses.get_mut(i).unwrap();
            status.cpu = cpu_usage;
        }
    }

    cli_table::print_stdout(service_statuses.with_title())?;

    Ok(())
}

/// Get systemd services having an extension `.ser.service`. We only monitor services created by this tool
async fn get_servicer_services() -> Result<Vec<String>, std::io::Error> {
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

/// Get CPU clock ticks per second. This value is usually 100 on x86_64
pub fn clock_ticks_per_second() -> u64 {
    unsafe { libc::sysconf(libc::_SC_CLK_TCK) as u64 }
}

/// Get CPU time for a vector of services
///
/// # Arguments
///
/// * `service_statuses`
///
pub async fn get_cpu_times(service_statuses: Vec<ServiceStatus>) -> Vec<u64> {
    futures::future::try_join_all(service_statuses.into_iter().map(|status| {
        tokio::spawn(async move {
            if status.active == "active" {
                get_cpu_time(status.pid).await.unwrap()
            } else {
                0
            }
        })
    }))
    .await
    .unwrap()
}
