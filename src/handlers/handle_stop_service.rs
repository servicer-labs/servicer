use crate::{
    handlers::handle_show_status::handle_show_status,
    utils::{
        service_names::{get_full_service_name, is_full_name},
        systemd::ManagerProxyBlocking,
    },
};

/// Stops a service
///
/// TODO support stopping all services with `all`
///
/// # Arguments
///
/// * `name`- Name of the service to stop in short form (hello-world) or long form (hello-world.stabled.service).
///
pub fn handle_stop_service(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let full_service_name = if is_full_name(&name) {
        name.clone()
    } else {
        get_full_service_name(&name)
    };

    let connection = zbus::blocking::Connection::system().unwrap();
    let manager_proxy = ManagerProxyBlocking::new(&connection).unwrap();
    stop_service(&manager_proxy, &full_service_name);

    println!("Stopped {name}");

    handle_show_status().unwrap();

    Ok(())
}

fn stop_service(manager_proxy: &ManagerProxyBlocking, full_service_name: &String) {
    manager_proxy
        .stop_unit(full_service_name.to_string(), "replace".into())
        .expect(&format!("Failed to stop service {full_service_name}"));
}
