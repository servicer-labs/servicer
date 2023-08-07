use std::path::{Path, PathBuf};

/// Shortens the service name from `example.ser.service` to `example`.
///
/// Must externally check whether `.ser.service` exists at the end otherwise this function
/// will throw an error
///
/// # Arguments
///
/// * `full_service_name`
///
pub fn get_short_service_name(full_service_name: &str) -> String {
    let file_extension = format!(".ser.service");

    full_service_name
        .trim_end_matches(file_extension.as_str())
        .to_string()
}

/// Returns the full service name, ending with `.ser.service`
///
/// Must externally ensure that `.ser.service` is already not present.
///
/// # Arguments
///
/// * `short_name`
///
pub fn get_full_service_name(short_name: &str) -> String {
    format!("{}.ser.service", short_name)
}

/// Whether it is a full service name, i.e. ending with `ser.service`
///
/// # Arguments
///
/// * `name` - The service name
///
pub fn is_full_name(name: &str) -> bool {
    let service_extension = format!(".ser.service");

    name.ends_with(&service_extension)
}

/// Get the path to a service file
///
/// # Arguments
///
/// * `full_service_name`
///
pub fn get_service_file_path(full_service_name: &str) -> PathBuf {
    Path::new("/etc/systemd/system/").join(full_service_name)
}
