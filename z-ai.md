# Simple Z.ai / GLM Quota Menu Bar App Notes

## Goal

Build a minimal macOS menu bar app that displays Z.ai / GLM quota left and reset times.

Menu bar:

```text
Z.ai 65%
```

Dropdown:

```text
Time quota: 0% left
Token quota: 65% left
Time reset: May 7, 13:13
Token reset: Apr 29, 10:56
Refresh now
Set API key
Quit
```

## Implementation Phases

- [x] Create Rust project scaffold.
- [x] Add AGENTS.md with simplicity and verification rules.
- [x] Add quota parser tests from the sample API response.
- [x] Implement quota parser and reset-time formatting.
- [x] Implement Keychain read/write through `/usr/bin/security`.
- [x] Implement quota fetch through `/usr/bin/curl`.
- [x] Implement AppKit status item and dropdown.
- [x] Build release binary.
- [x] Create `.app` bundle with `LSUIElement`.
- [x] Ad-hoc sign the `.app`.
- [x] Measure idle RSS after launch and after 5 minutes.

## API

Endpoint:

```text
GET https://api.z.ai/api/monitor/usage/quota/limit
Authorization: Bearer <api-key>
```

The endpoint is unofficial. If it fails, keep the last successful quota visible and show one concise error row.

GLM is included in public docs and metadata for discoverability because users search for GLM quota monitoring. The app still reads the Z.ai quota endpoint.

## Verification

Run:

```bash
cargo test
cargo build --release
scripts/package_app.sh
```

Memory target:

```bash
ps -o rss= -p <pid>
```

Record the measured RSS here before claiming the sub-10 MB target is met.

## Memory Measurements

The project goal is a tiny package and idle RAM as close to 10 MB as practical. The package goal is met; the strict sub-10 MB RSS goal is not met by the current AppKit build.

Initial launch measurement:

```text
PID 71508, RSS 40320 KB
```

First 5-minute measurement:

```text
PID 71508, RSS 11840 KB
```

After removing `chrono`, app bundle size dropped to about 660 KB.

Second launch measurement:

```text
PID 75357, RSS 40192 KB
```

Second 5-minute measurement:

```text
PID 75357, RSS 13952 KB
```

The current AppKit build does not meet the sub-10 MB RSS target. The remaining memory appears to be dominated by the macOS AppKit status item runtime rather than the Rust quota parser or HTTP path.

## Open Source Publishing Checklist

- [x] Public README with install, usage, and lightweight targets.
- [x] MIT license.
- [x] Security notes for API keys.
- [x] `.gitignore` excludes build artifacts and packaged app output.
- [ ] Add repository URL to `Cargo.toml` after creating the GitHub repository.
- [ ] Add screenshots after manually capturing the menu bar and dropdown.
- [ ] Create a first GitHub release with `dist/Z.ai Quota.app` zipped.
