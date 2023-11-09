use crate::{
    utils::service_names::get_full_service_name,
    utils::{
        service_actions::reload_service,
        systemd::{get_active_state, ManagerProxy},
    },
};

use super::handle_show_status::handle_show_status;

/// Reloads the unit of a failed service. The service state must be 'failed', otherwise the
/// systemd dbus API throws an error.
///
/// # Arguments
///
/// * `name` - The service name
///
pub async fn handle_reload_service(
    name: &String,
    show_status: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let connection = zbus::Connection::system().await?;
    let manager_proxy = ManagerProxy::new(&connection).await?;

    let full_service_name = get_full_service_name(&name);

    let active_state = get_active_state(&connection, &full_service_name).await;

    if active_state == "reloading" {
        eprintln!("No-op. Service {full_service_name} is already {active_state}");
    } else {
        reload_service(&manager_proxy, &full_service_name).await;
        println!("service reloaded: {name}");
    };

    if show_status {
        handle_show_status().await?;
    }

    Ok(())
}
