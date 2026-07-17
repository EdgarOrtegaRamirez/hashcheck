# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | Yes                |

## Reporting a Vulnerability

If you discover a security vulnerability, please report it privately:

1. Do **not** open a public issue
2. Email the maintainer or use GitHub's private vulnerability reporting feature

We will respond within 48 hours and work to resolve the issue promptly.

## Security Considerations

- All file paths are validated to prevent path traversal attacks
- Checksum file parsing validates hash lengths before selecting algorithms
- Dependencies are pinned to specific versions to avoid supply chain risks