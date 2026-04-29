use std::process::Command;

use crate::quota::{parse_quota_snapshot, QuotaSnapshot};

const QUOTA_URL: &str = "https://api.z.ai/api/monitor/usage/quota/limit";

pub fn fetch_quota(api_key: &str) -> Result<QuotaSnapshot, String> {
    let output = Command::new("/usr/bin/curl")
        .args([
            "--fail",
            "--silent",
            "--show-error",
            "--max-time",
            "15",
            "-H",
            &format!("Authorization: Bearer {api_key}"),
            QUOTA_URL,
        ])
        .output()
        .map_err(|error| format!("failed to run curl: {error}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(if stderr.is_empty() {
            format!("quota refresh failed with {}", output.status)
        } else {
            stderr
        });
    }

    let body = String::from_utf8(output.stdout)
        .map_err(|error| format!("quota response was not UTF-8: {error}"))?;
    parse_quota_snapshot(&body)
}
