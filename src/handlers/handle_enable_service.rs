use crate::handlers::handle_show_status::handle_show_status;
use crate::utils::service_actions::enable_service;
use crate::utils::{service_names::get_full_service_name, systemd::ManagerProxy};

/// Enables a service to start on boot
///
/// # Arguments
///
/// * `name`- Name of the service to stop
///
pub async fn handle_enable_service(
    name: &String,
    show_status: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let full_service_name = get_full_service_name(&name);

    let connection = zbus::Connection::system().await?;
    let manager_proxy = ManagerProxy::new(&connection).await?;

    enable_service(&manager_proxy, &full_service_name).await;

    // Reload necessary for UnitFileState to update
    manager_proxy.reload().await?;

    println!("Enabled {name}");

    if show_status {
        handle_show_status().await?;
    }

    Ok(())
}
