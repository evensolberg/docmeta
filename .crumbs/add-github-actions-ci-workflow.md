---
id: meta-ltd
title: Add GitHub Actions CI workflow
status: closed
type: feature
priority: 1
tags:
- ci
- testing
created: 2026-04-03
updated: 2026-04-04
dependencies: []
---

# Add GitHub Actions CI workflow

No GitHub Actions workflow exists. Add `.github/workflows/ci.yml` that runs on every PR and push to main: `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`. Dependabot is already configured but there is nothing to validate PRs it opens.
