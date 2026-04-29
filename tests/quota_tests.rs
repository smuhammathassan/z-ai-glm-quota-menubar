use z_ai_quota_menubar::quota::{menu_bar_title, parse_quota_snapshot, quota_left_label};

const SAMPLE: &str = r#"{
  "code": 200,
  "msg": "Operation successful",
  "data": {
    "limits": [
      {
        "type": "TIME_LIMIT",
        "unit": 5,
        "number": 1,
        "usage": 100,
        "currentValue": 100,
        "remaining": 0,
        "percentage": 100,
        "nextResetTime": 1778144000998,
        "usageDetails": [
          { "modelCode": "search-prime", "usage": 55 },
          { "modelCode": "web-reader", "usage": 45 },
          { "modelCode": "zread", "usage": 0 }
        ]
      },
      {
        "type": "TOKENS_LIMIT",
        "unit": 3,
        "number": 5,
        "percentage": 35,
        "nextResetTime": 1777444613249
      }
    ],
    "level": "lite"
  },
  "success": true
}"#;

#[test]
fn parses_time_and_token_left_from_sample() {
    let snapshot = parse_quota_snapshot(SAMPLE).expect("sample parses");

    assert_eq!(snapshot.time_left_percent, Some(0));
    assert_eq!(snapshot.token_left_percent, Some(65));
    assert_eq!(menu_bar_title(Some(&snapshot)), "Z.ai 65%");
    assert_eq!(quota_left_label(snapshot.time_left_percent), "0% left");
    assert_eq!(quota_left_label(snapshot.token_left_percent), "65% left");
}

#[test]
fn formats_reset_times_as_short_local_labels() {
    let snapshot = parse_quota_snapshot(SAMPLE).expect("sample parses");

    let time_reset = snapshot.time_reset.expect("time reset");
    let token_reset = snapshot.token_reset.expect("token reset");

    assert!(time_reset.contains("May 7"), "{time_reset}");
    assert!(token_reset.contains("Apr 29"), "{token_reset}");
    assert!(time_reset.contains(':'), "{time_reset}");
    assert!(token_reset.contains(':'), "{token_reset}");
}

#[test]
fn missing_limits_render_as_unknown() {
    let snapshot = parse_quota_snapshot(r#"{"code":200,"data":{"limits":[]},"success":true}"#)
        .expect("empty limits parse");

    assert_eq!(snapshot.time_left_percent, None);
    assert_eq!(snapshot.token_left_percent, None);
    assert_eq!(menu_bar_title(Some(&snapshot)), "Z.ai --%");
    assert_eq!(quota_left_label(snapshot.time_left_percent), "--% left");
}

#[test]
fn rejects_malformed_json() {
    let error = parse_quota_snapshot("{bad json").expect_err("malformed json fails");

    assert!(error.contains("invalid quota response"), "{error}");
}
