# Security Policy

[English](./SECURITY.md) | [简体中文](./SECURITY.zh-CN.md)

## Supported Versions

This project is still in an early stage. Security fixes are expected to land on the default active development branch first.

## Reporting a Vulnerability

Please do not publish full vulnerability details in a public issue.

Preferred order:

1. Use GitHub Security Advisories or private vulnerability reporting if it is enabled for the repository.
2. If no private reporting channel is available, contact the maintainer privately.
3. If neither option is available, open a minimal public issue only to request a secure contact path. Do not include exploit details, credentials, or server-specific data.

## What to Include

A good report usually contains:

- affected area or file
- impact summary
- reproduction steps
- proof-of-concept details when safe to share privately
- version, commit, or branch information
- any mitigation ideas you have already tested

## Response Expectations

The goal is to:

- acknowledge receipt within 3 business days when possible
- reproduce and assess severity
- prepare a fix and validation plan
- coordinate disclosure after a patch is available

## Sensitive Areas in This Project

Please take extra care when reporting issues related to:

- SSH authentication or host verification
- remote file writes and backup handling
- config composition and rollback behavior
- subscription parsing of untrusted input
- command execution on the managed server
