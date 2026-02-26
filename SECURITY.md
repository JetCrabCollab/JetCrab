# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.4.x   | Yes                |
| < 0.4   | No                 |

## Reporting a Vulnerability

If you discover a security vulnerability in JetCrab, please report it privately:

1. **Do not** open a public GitHub issue.
2. Email security concerns to: team@jetcrab.dev (or create a private security advisory on GitHub).
3. Include a clear description of the vulnerability and steps to reproduce.
4. Allow reasonable time for a fix before public disclosure.

We will acknowledge receipt within 48 hours and provide updates on the fix timeline.

## Security Practices

- Dependencies are monitored with `cargo audit`.
- Security updates are prioritized in release planning.
- Untrusted JavaScript runs in a sandboxed WASM environment via Chitin.
