use std::process::Command;

const SERVICE: &str = "z-ai-quota-menubar";
const ACCOUNT: &str = "default";

pub fn read_api_key() -> Option<String> {
    let output = Command::new("/usr/bin/security")
        .args(["find-generic-password", "-s", SERVICE, "-a", ACCOUNT, "-w"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let key = String::from_utf8(output.stdout).ok()?.trim().to_string();
    (!key.is_empty()).then_some(key)
}

pub fn write_api_key(api_key: &str) -> Result<(), String> {
    let _ = Command::new("/usr/bin/security")
        .args(["delete-generic-password", "-s", SERVICE, "-a", ACCOUNT])
        .output();

    let status = Command::new("/usr/bin/security")
        .args([
            "add-generic-password",
            "-s",
            SERVICE,
            "-a",
            ACCOUNT,
            "-w",
            api_key,
            "-U",
        ])
        .status()
        .map_err(|error| format!("failed to run security: {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("security exited with {status}"))
    }
}
