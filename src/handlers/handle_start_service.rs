use crate::{
    utils::service_names::{get_full_service_name, is_full_name},
    utils::systemd::{get_active_state, get_unit_file_state, ManagerProxyBlocking},
};

/// Starts a systemd service. This is a no-op if the service is already running.
///
/// # Arguments
///
/// * `name` - The service name. This can be in short form, i.e. 'hello-world' or long form 'hello-world.stabled.service'
/// * `enable_on_boot` - Enable the service to start on boot. A running service can be enabled to start on boot.
///
pub fn handle_start_service(
    name: String,
    enable_on_boot: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let connection = zbus::blocking::Connection::system().unwrap();
    let manager_proxy = ManagerProxyBlocking::new(&connection).unwrap();

    let full_name = if is_full_name(&name) {
        name.clone()
    } else {
        get_full_service_name(&name)
    };

    let active_state = get_active_state(&connection, &full_name);

    if active_state == "active" || active_state == "reloading" {
        if enable_on_boot {
            let unit_file_state = get_unit_file_state(&connection, &full_name);
            if unit_file_state == "enabled" {
                eprintln!(
                    "No-op. Service {full_name} is already {active_state} and {unit_file_state}"
                );
            } else {
                enable_service(&manager_proxy, &full_name);
                println!("Running service {full_name} enabled on boot");
            }
        } else {
            eprintln!("No-op. Service {full_name} is already {active_state}");
        }
    } else {
        let start_service_result = manager_proxy
            .start_unit(full_name.clone(), "replace".into())
            .expect(&format!("Failed to start service {name}"));

        if enable_on_boot {
            enable_service(&manager_proxy, &full_name);
        }
        println!("service started: {start_service_result}. Enable on boot: {enable_on_boot}");
    };

    // TODO show status

    Ok(())
}

/// Enables a service on boot
///
/// # Arguments
///
/// * `manager_proxy`: Blocking Manager proxy object
/// * `full_service_name`: Full name of the service, having '.stabled.service' at the end
///
fn enable_service(manager_proxy: &ManagerProxyBlocking, full_service_name: &String) {
    manager_proxy
        .enable_unit_files(vec![full_service_name.clone()], false, true)
        .expect(&format!("Failed to enable service {full_service_name}"));
}
