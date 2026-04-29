use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotaSnapshot {
    pub time_left_percent: Option<u8>,
    pub token_left_percent: Option<u8>,
    pub time_reset: Option<String>,
    pub token_reset: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: Option<ResponseData>,
}

#[derive(Debug, Deserialize)]
struct ResponseData {
    limits: Vec<Limit>,
}

#[derive(Debug, Deserialize)]
struct Limit {
    #[serde(rename = "type")]
    kind: String,
    remaining: Option<i64>,
    percentage: Option<i64>,
    #[serde(rename = "nextResetTime")]
    next_reset_time: Option<i64>,
}

pub fn parse_quota_snapshot(json: &str) -> Result<QuotaSnapshot, String> {
    let response: ApiResponse =
        serde_json::from_str(json).map_err(|error| format!("invalid quota response: {error}"))?;

    let mut snapshot = QuotaSnapshot {
        time_left_percent: None,
        token_left_percent: None,
        time_reset: None,
        token_reset: None,
    };

    for limit in response.data.map(|data| data.limits).unwrap_or_default() {
        let left = left_percent(limit.remaining, limit.percentage);
        let reset = limit.next_reset_time.and_then(format_reset_time);

        match limit.kind.as_str() {
            "TIME_LIMIT" => {
                snapshot.time_left_percent = left;
                snapshot.time_reset = reset;
            }
            "TOKENS_LIMIT" => {
                snapshot.token_left_percent = left;
                snapshot.token_reset = reset;
            }
            _ => {}
        }
    }

    Ok(snapshot)
}

pub fn menu_bar_text(snapshot: Option<&QuotaSnapshot>) -> String {
    match snapshot.and_then(|s| s.token_left_percent) {
        Some(left) => format!("{left}%"),
        None => "--%".to_string(),
    }
}

pub fn quota_left_label(value: Option<u8>) -> String {
    match value {
        Some(left) => format!("{left}% left"),
        None => "--% left".to_string(),
    }
}

fn left_percent(remaining: Option<i64>, percentage: Option<i64>) -> Option<u8> {
    let value = remaining.or_else(|| percentage.map(|used| 100 - used))?;
    Some(value.clamp(0, 100) as u8)
}

fn format_reset_time(milliseconds: i64) -> Option<String> {
    let seconds = (milliseconds / 1000) as libc::time_t;
    let mut time = std::mem::MaybeUninit::<libc::tm>::uninit();
    let time = unsafe {
        let result = libc::localtime_r(&seconds, time.as_mut_ptr());
        if result.is_null() {
            return None;
        }
        time.assume_init()
    };

    let mut buffer = [0_i8; 32];
    let written = unsafe {
        libc::strftime(
            buffer.as_mut_ptr(),
            buffer.len(),
            c"%b %-d, %H:%M".as_ptr(),
            &time,
        )
    };
    if written == 0 {
        return None;
    }

    let value = unsafe { std::ffi::CStr::from_ptr(buffer.as_ptr()) };
    Some(value.to_string_lossy().into_owned())
}
