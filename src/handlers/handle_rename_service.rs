use crate::{
    handlers::handle_show_status::handle_show_status,
    utils::{
        service_names::{get_full_service_name, get_service_file_path},
        systemd::{get_active_state, get_unit_file_state},
    },
};

use zbus::Connection;

use super::{
    handle_delete_service::handle_delete_service, handle_enable_service::handle_enable_service,
    handle_start_service::handle_start_service,
};

/// Renames a service. A running service will be restarted
///
/// Under the hood the exiting service is stopped and deleted. A new service file
/// with same contents is created.
///
/// # Arguments
///
/// * `name`- Name of the service to restart
/// * `new_name` - New name
///
pub async fn handle_rename_service(
    name: &String,
    new_name: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create new service file
    let full_service_name = get_full_service_name(&name);
    let service_file_path = get_service_file_path(&full_service_name);
    let service_file_path_str = service_file_path.to_str().unwrap().to_string();

    let new_full_service_name = get_full_service_name(&new_name);
    let new_service_file_path = get_service_file_path(&new_full_service_name);
    let new_service_file_path_str = new_service_file_path.to_str().unwrap().to_string();

    // Copy .service file
    tokio::fs::copy(service_file_path_str, new_service_file_path_str).await?;

    // Read active and unit state of current service
    let connection = Connection::system().await?;
    let active_state: String = get_active_state(&connection, &full_service_name).await;
    let unit_state = get_unit_file_state(&connection, &full_service_name).await;

    // Delete existing service
    handle_delete_service(name, false).await?;

    if active_state == "active" {
        handle_start_service(new_name, false).await?;
    }

    if unit_state == "enabled" {
        handle_enable_service(new_name, false).await?;
    }

    handle_show_status().await?;

    Ok(())
}
