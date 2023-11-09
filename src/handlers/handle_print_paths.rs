use cli_table::{Table, WithTitle};

use crate::utils::{
    service_names::{get_full_service_name, get_service_file_path},
    systemd::get_unit_path,
};

#[derive(Table, Clone)]
pub struct PathStatus {
    // The file name
    name: String,

    // The file path
    path: String,
}

/// Locate files used by a service and print their paths. Displays the .service path and unit path
/// if the service is enabled
///
/// # Arguments
///
/// * `name` - The service name
///
pub async fn handle_print_paths(name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut path_details = Vec::<PathStatus>::new();

    let full_service_name = get_full_service_name(name);
    let service_file_path = get_service_file_path(&full_service_name);

    if service_file_path.exists() {
        println!("Paths for {}:", full_service_name);

        // 1. Service file path
        path_details.push(PathStatus {
            name: "Service file".to_string(),
            path: service_file_path.to_str().unwrap().to_string(),
        });

        // 2. Unit file
        path_details.push(PathStatus {
            name: "Unit file".to_string(),
            path: get_unit_path(&full_service_name),
        });

        cli_table::print_stdout(path_details.with_title())?;
    } else {
        eprintln!("No such service {}", full_service_name);
    }

    Ok(())
}
