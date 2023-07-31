use crate::TOOL_NAME;

/// Shortens the service name from `example.stabled.service` to `example`.
///
/// Must externally check whether `.stabled.service` exists at the end otherwise this function
/// will throw an error
///
/// # Arguments
///
/// * `full_service_name`
///
pub fn get_short_service_name(full_service_name: &str) -> String {
    let file_extension = format!(".{TOOL_NAME}.service");

    full_service_name
        .trim_end_matches(file_extension.as_str())
        .to_string()
}

/// Returns the full service name, ending with `.stabled.service`
///
/// Must externally ensure that `.stabled.service` is already not present.
///
/// # Arguments
///
/// * `short_name`
///
pub fn get_full_service_name(short_name: &str) -> String {
    format!("{}.{}.service", short_name, TOOL_NAME)
}

/// Whether it is a full service name, i.e. ending with `stabled.service`
///
/// # Arguments
///
/// * `name` - The service name
///
pub fn is_full_name(name: &str) -> bool {
    let service_extension = format!(".{TOOL_NAME}.service");

    name.ends_with(&service_extension)
}
