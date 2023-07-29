use zbus::blocking::Connection;
use zbus::dbus_proxy;

/// Proxy object for `org.freedesktop.systemd1.Manager`.
/// Taken from https://github.com/lucab/zbus_systemd/blob/main/src/systemd1/generated.rs
#[dbus_proxy(
    interface = "org.freedesktop.systemd1.Manager",
    default_service = "org.freedesktop.systemd1",
    default_path = "/org/freedesktop/systemd1"
)]
pub trait Manager {
    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#StartUnit()) Call interface method `StartUnit`.
    #[dbus_proxy(name = "StartUnit")]
    fn start_unit(
        &self,
        name: String,
        mode: String,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#EnableUnitFiles()) Call interface method `EnableUnitFiles`.
    #[dbus_proxy(name = "EnableUnitFiles")]
    fn enable_unit_files(
        &self,
        files: Vec<String>,
        runtime: bool,
        force: bool,
    ) -> zbus::Result<(bool, Vec<(String, String, String)>)>;
}

/// Proxy object for `org.freedesktop.systemd1.Unit`.
/// Taken from https://github.com/lucab/zbus_systemd/blob/main/src/systemd1/generated.rs
#[dbus_proxy(
    interface = "org.freedesktop.systemd1.Unit",
    default_service = "org.freedesktop.systemd1",
    assume_defaults = false
)]
pub trait Unit {
    /// Get property `ActiveState`.
    #[dbus_proxy(property)]
    fn active_state(&self) -> zbus::Result<String>;

    /// Get property `LoadState`.
    #[dbus_proxy(property)]
    fn load_state(&self) -> zbus::Result<String>;

    /// Get property `UnitFileState`.
    #[dbus_proxy(property)]
    fn unit_file_state(&self) -> zbus::Result<String>;
}

/// Returns the load state of a systemd unit
///
/// Returns `invalid-unit-path` if the path is invalid
///
/// # Arguments
///
/// * `full_service_name`: Full name of the service name with '.service' in the end
/// * `connection`: Blocking zbus connection
///
pub fn get_load_state(full_service_name: &String, connection: &Connection) -> String {
    // Object path is different from file path which begins with /etc/systemd/system
    let object_path = format!(
        "/org/freedesktop/systemd1/unit/{}",
        encode_as_dbus_object_path(full_service_name)
    );

    match zbus::zvariant::ObjectPath::try_from(object_path) {
        Ok(path) => {
            let unit_proxy = UnitProxyBlocking::new(connection, path).unwrap();
            unit_proxy
                .load_state()
                .unwrap_or("invalid-unit-path".into())
        }
        Err(_) => "invalid-unit-path".to_string(),
    }
}

/// Returns the load state of a systemd unit
///
/// Returns `invalid-unit-path` if the path is invalid
///
/// # Arguments
///
/// * `full_service_name`: Full name of the service name with '.service' in the end
/// * `connection`: Blocking zbus connection
///
pub fn get_active_state(full_service_name: &String, connection: &Connection) -> String {
    let object_path = format!(
        "/org/freedesktop/systemd1/unit/{}",
        encode_as_dbus_object_path(full_service_name)
    );

    match zbus::zvariant::ObjectPath::try_from(object_path) {
        Ok(path) => {
            let unit_proxy = UnitProxyBlocking::new(connection, path).unwrap();
            unit_proxy
                .active_state()
                .unwrap_or("invalid-unit-path".into())
        }
        Err(_) => "invalid-unit-path".to_string(),
    }
}

/// Returns the unit file state of a systemd unit. If the state is `enabled`, the unit loads on every boot
///
/// Returns `invalid-unit-path` if the path is invalid
///
/// # Arguments
///
/// * `full_service_name`: Full name of the service name with '.service' in the end
/// * `connection`: Blocking zbus connection
///
pub fn get_unit_file_state(full_service_name: &String, connection: &Connection) -> String {
    let object_path = format!(
        "/org/freedesktop/systemd1/unit/{}",
        encode_as_dbus_object_path(full_service_name)
    );

    match zbus::zvariant::ObjectPath::try_from(object_path) {
        Ok(path) => {
            let unit_proxy = UnitProxyBlocking::new(connection, path).unwrap();
            unit_proxy
                .unit_file_state()
                .unwrap_or("invalid-unit-path".into())
        }
        Err(_) => "invalid-unit-path".to_string(),
    }
}

/// Encode into a valid dbus string
///
/// # Arguments
///
/// * `input_string`
///
fn encode_as_dbus_object_path(input_string: &str) -> String {
    input_string
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '/' || c == '_' {
                c.to_string()
            } else {
                format!("_{:x}", c as u32)
            }
        })
        .collect()
}
