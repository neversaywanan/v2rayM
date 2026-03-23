# Contributing to v2rayM

[English](./CONTRIBUTING.md) | [简体中文](./CONTRIBUTING.zh-CN.md)

Thanks for considering contributing to v2rayM.

This document explains how we keep changes easy to review, safe to test, and aligned with the direction of the project.

## Ground Rules

- Keep pull requests focused. One problem, one change set.
- Prefer small, reviewable commits over large mixed refactors.
- If a change affects user behavior, update the relevant documentation in the same pull request.
- When changing SSH, config mutation, or subscription parsing logic, be extra careful about regressions.
- Never commit secrets, real server credentials, or production subscription URLs.

## Development Setup

### Prerequisites

- Node.js 20+
- npm 10+
- Rust stable
- Tauri prerequisites for your OS

### Install dependencies

```bash
npm install
```

### Start the app in development mode

```bash
npm run tauri dev
```

### Run frontend tests

```bash
npm test
```

### Run Rust tests

```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

### Build the application

```bash
npm run tauri build
```

## Branch Naming

Use clear branch names when possible.

Examples:

- `feat/subscription-bulk-sync`
- `fix/ssh-timeout-handling`
- `docs/bilingual-docs`
- `refactor/config-composer`

## Commit Messages

Conventional Commits are recommended.

Examples:

- `feat: support bulk applying subscription nodes`
- `fix: preserve display names when switching outbound`
- `docs: add bilingual repository documentation`
- `test: add parser coverage for trojan links`

## Pull Request Checklist

Before opening a PR, please make sure:

- The branch is up to date with the target branch.
- The change has a clear reason and a scoped description.
- Tests were added or updated when behavior changed.
- Local tests were run when relevant.
- Screenshots are included for visible UI changes.
- New strings are added to both language tables when needed.
- Documentation is updated if setup, behavior, or project structure changed.

## Coding Guidelines

### Frontend

- Keep React components readable and avoid mixing unrelated UI concerns.
- Reuse store and utility helpers instead of duplicating client-side logic.
- Add tests for utility functions when introducing non-trivial string or parsing logic.
- If you add user-facing copy, update both `zh` and `en` translations.

### Rust / Tauri

- Keep command handlers narrow and move reusable logic into focused modules.
- Prefer explicit error mapping over silent failure when the user can act on it.
- Preserve existing config structure when composing or mutating remote JSON.
- Add or update tests for parser, config composer, and error behavior when touching backend logic.

## Testing Expectations

Not every documentation-only change needs a full app build, but code changes should include appropriate verification.

Recommended checks:

```bash
npm test
cargo test --manifest-path src-tauri/Cargo.toml
npm run tauri build
```

Run the checks that match the scope of your change. If you skip any, mention that in the pull request description.

## Reporting Bugs

When filing a bug report, include:

- what you expected to happen
- what actually happened
- your OS and app environment
- whether the target server is running V2Ray and `systemd`
- reproduction steps
- relevant logs or screenshots with secrets removed

## Suggesting Features

Feature requests are welcome, especially if they improve:

- safety of remote config editing
- compatibility with common subscription formats
- onboarding and error messages
- packaging, release, and cross-platform experience

## Areas That Need Extra Care

Please open a discussion or draft PR first for larger changes involving:

- config composition behavior
- rollback semantics
- parser compatibility changes
- SSH transport or authentication flow
- cross-platform packaging and signing

## Questions

If something is unclear, open an issue with context and your proposed direction. Clear intent early usually saves time for everyone.
