use super::systemd::ManagerProxy;

/// Starts a service
///
/// # Arguments
///
/// * `manager_proxy`: Manager proxy object
/// * `full_service_name`: Full name of the service, having '.ser.service' at the end
///
pub async fn start_service(manager_proxy: &ManagerProxy<'_>, full_service_name: &String) -> String {
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
/// * `manager_proxy`: Manager proxy object
/// * `full_service_name`: Full name of the service, having '.ser.service' at the end
///
pub async fn enable_service(
    manager_proxy: &ManagerProxy<'_>,
    full_service_name: &String,
) -> (bool, Vec<(String, String, String)>) {
    manager_proxy
        .enable_unit_files(vec![full_service_name.clone()], false, true)
        .await
        .expect(&format!(
            "Failed to enable service {full_service_name}. Retry in sudo mode."
        ))
}

pub async fn stop_service(manager_proxy: &ManagerProxy<'_>, full_service_name: &String) {
    manager_proxy
        .stop_unit(full_service_name.to_string(), "replace".into())
        .await
        .expect(&format!("Failed to stop service {full_service_name}"));
}

/// Reloads the unit of a failed service
///
/// # Arguments
///
/// * `manager_proxy`: Manager proxy object
/// * `full_service_name`: Full name of the service, having '.ser.service' at the end
///
pub async fn reload_service(manager_proxy: &ManagerProxy<'_>, full_service_name: &String) -> () {
    manager_proxy
        .reload_unit(full_service_name.clone(), "replace".into())
        .await
        .expect(&format!(
            "Failed to reload service {full_service_name}. Ensure it has an ExecReload statement"
        ));
}

/// Disables a service on boot
///
/// # Arguments
///
/// * `manager_proxy`: Manager proxy object
/// * `full_service_name`: Full name of the service, having '.ser.service' at the end
///
pub async fn disable_service(manager_proxy: &ManagerProxy<'_>, full_service_name: &String) {
    manager_proxy
        .disable_unit_files(vec![full_service_name.clone()], false)
        .await
        .expect(&format!(
            "Failed to disable service {full_service_name}. Retry in sudo mode."
        ));
}
