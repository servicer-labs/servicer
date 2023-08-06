use crate::handlers::handle_show_status::handle_show_status;
use crate::utils::{service_names::get_full_service_name, systemd::ManagerProxy};

/// Disables a service from starting on boot
///
/// # Arguments
///
/// * `name`- Name of the service to disable
///
pub async fn handle_disable_service(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let full_service_name = get_full_service_name(&name);

    let connection = zbus::Connection::system().await.unwrap();
    let manager_proxy = ManagerProxy::new(&connection).await.unwrap();

    disable_service(&manager_proxy, &full_service_name).await;

    // Reload necessary for UnitFileState to update
    manager_proxy.reload().await.unwrap();

    println!("Disabled {name}");

    handle_show_status().await.unwrap();

    Ok(())
}

/// Disables a service on boot
///
/// # Arguments
///
/// * `manager_proxy`: Blocking Manager proxy object
/// * `full_service_name`: Full name of the service, having '.ser.service' at the end
///
async fn disable_service(manager_proxy: &ManagerProxy<'_>, full_service_name: &String) {
    manager_proxy
        .disable_unit_files(vec![full_service_name.clone()], false)
        .await
        .expect(&format!(
            "Failed to disable service {full_service_name}. Retry in sudo mode."
        ));
}
