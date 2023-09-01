use crate::utils::service_names::{get_full_service_name, get_service_file_path};

use super::{
    handle_disable_service::handle_disable_service, handle_show_status::handle_show_status,
    handle_stop_service::handle_stop_service,
};

/// Deletes a service, stopping and disabling it if necessary and removing the .service file
///
/// # Arguments
///
/// * `name`- Name of the service to stop
///
pub async fn handle_delete_service(name: String) -> Result<(), Box<dyn std::error::Error>> {
    handle_stop_service(name.clone(), false).await?;
    handle_disable_service(name.clone(), false).await?;

    let full_service_name = get_full_service_name(&name);
    let service_file_path = get_service_file_path(&full_service_name);
    let service_file_path_str = service_file_path.to_str().unwrap().to_string();

    // Delete .service file
    tokio::fs::remove_file(&service_file_path).await?;

    println!("Deleted {service_file_path_str}");

    handle_show_status().await?;

    Ok(())
}
