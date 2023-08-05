use crate::{
    utils::service_names::get_full_service_name,
    utils::systemd::{get_active_state, get_unit_file_state, ManagerProxy},
};

/// Starts a systemd service. This is a no-op if the service is already running.
///
/// # Arguments
///
/// * `name` - The service name
/// * `enable_on_boot` - Enable the service to start on boot. A running service can be enabled to start on boot.
///
pub async fn handle_start_service(
    name: String,
    enable_on_boot: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let connection = zbus::Connection::system().await.unwrap();
    let manager_proxy = ManagerProxy::new(&connection).await.unwrap();

    let full_name = get_full_service_name(&name);

    let active_state = get_active_state(&connection, &full_name).await;

    if active_state == "active" || active_state == "reloading" {
        if enable_on_boot {
            let unit_file_state = get_unit_file_state(&connection, &full_name).await;
            if unit_file_state == "enabled" {
                eprintln!(
                    "No-op. Service {full_name} is already {active_state} and {unit_file_state}"
                );
            } else {
                enable_service(&manager_proxy, &full_name).await;
                println!("Running service {full_name} enabled on boot");
            }
        } else {
            eprintln!("No-op. Service {full_name} is already {active_state}");
        }
    } else {
        let start_service_result = start_service(&manager_proxy, &full_name).await;

        if enable_on_boot {
            enable_service(&manager_proxy, &full_name).await;
        }
        println!("service started: {start_service_result}");
    };

    // TODO show status

    Ok(())
}

/// Starts a service
///
/// # Arguments
///
/// * `manager_proxy`: Blocking Manager proxy object
/// * `full_service_name`: Full name of the service, having '.stabled.service' at the end
///
async fn start_service(manager_proxy: &ManagerProxy<'_>, full_service_name: &String) -> String {
    manager_proxy
        .start_unit(full_service_name.clone(), "replace".into())
        .await
        .expect(&format!("Failed to start service {full_service_name}"))
        .to_string()
}

/// Enables a service on boot
///
/// # Arguments
///
/// * `manager_proxy`: Blocking Manager proxy object
/// * `full_service_name`: Full name of the service, having '.stabled.service' at the end
///
async fn enable_service(manager_proxy: &ManagerProxy<'_>, full_service_name: &String) {
    manager_proxy
        .enable_unit_files(vec![full_service_name.clone()], false, true)
        .await
        .expect(&format!("Failed to enable service {full_service_name}"));
}
