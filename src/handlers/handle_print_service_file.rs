use tokio::{fs, io::AsyncReadExt};

use crate::utils::service_names::{get_full_service_name, get_service_file_path};

/// Print contents of a .service file
///
/// # Arguments
///
/// * `name` - The service name
///
pub async fn handle_print_service_file(name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let full_service_name = get_full_service_name(&name);
    let service_file_path = get_service_file_path(&full_service_name);

    if service_file_path.exists() {
        // Open the file using Tokio's File API
        let mut file = fs::File::open(&service_file_path).await?;

        // Create a buffer to hold the file contents
        let mut buffer = Vec::new();

        // Read the entire contents of the file into the buffer asynchronously
        file.read_to_end(&mut buffer).await?;

        // Convert the buffer to a UTF-8 string and print it
        let contents = String::from_utf8(buffer)?;
        println!(
            "Reading {}:\n{}",
            service_file_path.to_str().unwrap(),
            contents
        );
    } else {
        eprintln!("{}: No such file", service_file_path.to_str().unwrap());
    }

    Ok(())
}
