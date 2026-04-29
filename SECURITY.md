# Security Policy

## API Keys

Z.ai / GLM Quota Menubar stores API keys in macOS Keychain using the service name:

```text
z-ai-quota-menubar
```

The app does not write API keys to project files, logs, or local databases.

## Reporting Security Issues

If you find a security issue, do not open a public issue with secrets or exploit details. Contact the maintainer privately through the repository security contact once one is configured.

## Accidental Key Disclosure

If a key is pasted into a public issue, chat, screenshot, terminal log, or commit:

1. Revoke or rotate the key in Z.ai.
2. Remove the exposed value from public places where possible.
3. Treat the old key as compromised.
