use crate::{
    utils::service_names::get_full_service_name,
    utils::systemd::{get_active_state, ManagerProxy},
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
    name: String,
    show_status: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let connection = zbus::Connection::system().await.unwrap();
    let manager_proxy = ManagerProxy::new(&connection).await.unwrap();

    let full_service_name = get_full_service_name(&name);

    let active_state = get_active_state(&connection, &full_service_name).await;

    if active_state == "failed" {
        reload_service(&manager_proxy, &full_service_name).await;
        println!("service reloaded: {name}");
    } else {
        eprintln!("No-op. Service state of {full_service_name} is {active_state}");
    };

    if show_status {
        handle_show_status().await.unwrap();
    }

    Ok(())
}

/// Reloads the unit of a failed service
///
/// # Arguments
///
/// * `manager_proxy`: Manager proxy object
/// * `full_service_name`: Full name of the service, having '.ser.service' at the end
///
async fn reload_service(manager_proxy: &ManagerProxy<'_>, full_service_name: &String) -> () {
    manager_proxy
        .reload_unit(full_service_name.clone(), "replace".into())
        .await
        .expect(&format!("Failed to reload service {full_service_name}"));
}
