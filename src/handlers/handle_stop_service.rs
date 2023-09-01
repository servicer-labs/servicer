use crate::{
    handlers::handle_show_status::handle_show_status,
    utils::{service_names::get_full_service_name, systemd::ManagerProxy},
};

/// Stops a service
///
/// TODO support stopping all services with `all`
///
/// # Arguments
///
/// * `name`- Name of the service to stop
///
pub async fn handle_stop_service(
    name: String,
    show_status: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let full_service_name = get_full_service_name(&name);

    let connection = zbus::Connection::system().await?;
    let manager_proxy = ManagerProxy::new(&connection).await?;
    stop_service(&manager_proxy, &full_service_name).await;

    println!("Stopped {name}");

    if show_status {
        handle_show_status().await?;
    }

    Ok(())
}

async fn stop_service(manager_proxy: &ManagerProxy<'_>, full_service_name: &String) {
    manager_proxy
        .stop_unit(full_service_name.to_string(), "replace".into())
        .await
        .expect(&format!("Failed to stop service {full_service_name}"));
}
