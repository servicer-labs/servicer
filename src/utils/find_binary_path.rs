use tokio::process::Command;

/**
 * Runs `which` as SUDO_USER to find the path of the given binary.
 *
 * `ser create` must be called in sudo mode. The variable $PATH in sudo mode doesn't
 * hold most of the paths available to the regular user. Therefore we must call `which`
 * as SUDO_USER.
 *
 * # Arguments
 *
 * * `binary_name`- Find path for this interpreter
 * * `user` - Lookup as this user
 */
pub async fn find_binary_path(
    binary_name: &str,
    user: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Runs sudo -u hp bash -i -c "which deno"
    let output = Command::new("sudo")
        .arg("-u")
        .arg(user)
        .arg("bash")
        .arg("-i")
        .arg("-c")
        .arg(format!("which {binary_name}"))
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    if !output.status.success() {
        panic!("Failed to find {binary_name} in PATH")
    }

    Ok(stdout)
}
