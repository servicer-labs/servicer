use crate::{
    utils::service_names::get_full_service_name,
    utils::systemd::{get_active_state, ManagerProxy},
};

use super::handle_show_status::handle_show_status;

/// Starts a systemd service. This is a no-op if the service is already running.
///
/// # Arguments
///
/// * `name` - The service name
///
pub async fn handle_start_service(
    name: String,
    show_status: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let connection = zbus::Connection::system().await?;
    let manager_proxy = ManagerProxy::new(&connection).await?;

    let full_service_name = get_full_service_name(&name);

    let active_state = get_active_state(&connection, &full_service_name).await;

    if active_state == "active" || active_state == "reloading" {
        eprintln!("No-op. Service {full_service_name} is already {active_state}");
    } else {
        let start_service_result = start_service(&manager_proxy, &full_service_name).await;

        println!("service started: {start_service_result}");
    };

    if show_status {
        handle_show_status().await?;
    }

    Ok(())
}

/// Starts a service
///
/// # Arguments
///
/// * `manager_proxy`: Manager proxy object
/// * `full_service_name`: Full name of the service, having '.ser.service' at the end
///
async fn start_service(manager_proxy: &ManagerProxy<'_>, full_service_name: &String) -> String {
    manager_proxy
        .start_unit(full_service_name.clone(), "replace".into())
        .await
        .expect(&format!("Failed to start service {full_service_name}"))
        .to_string()
}
