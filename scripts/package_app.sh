#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
APP="$ROOT/dist/Z.ai Quota.app"
CONTENTS="$APP/Contents"
MACOS="$CONTENTS/MacOS"
RESOURCES="$CONTENTS/Resources"

cargo build --release --manifest-path "$ROOT/Cargo.toml"

rm -rf "$APP"
mkdir -p "$MACOS"
mkdir -p "$RESOURCES"
cp "$ROOT/target/release/z-ai-quota-menubar" "$MACOS/Z.ai Quota"
cp "$ROOT/assets/z-ai-logo.png" "$RESOURCES/z-ai-logo.png"

cat > "$CONTENTS/Info.plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key>
  <string>Z.ai Quota</string>
  <key>CFBundleIdentifier</key>
  <string>local.z-ai.quota-menubar</string>
  <key>CFBundleName</key>
  <string>Z.ai Quota</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>0.1.0</string>
  <key>CFBundleVersion</key>
  <string>1</string>
  <key>LSMinimumSystemVersion</key>
  <string>13.0</string>
  <key>LSUIElement</key>
  <true/>
</dict>
</plist>
PLIST

codesign --force --deep --sign - "$APP"
echo "$APP"
