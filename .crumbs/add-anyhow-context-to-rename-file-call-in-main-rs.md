---
id: meta-qw9
title: Add anyhow::Context to rename_file call in main.rs
status: closed
type: feature
priority: 3
tags:
- error-handling
- main
created: 2026-04-04
updated: 2026-04-04
dependencies: []
---

# Add anyhow::Context to rename_file call in main.rs

The three metadata calls in run() use .with_context(|| format!(...)) but the rename_file::rename_file call on line 93 propagates bare. RenameError::RenameFailed already carries from/to in its Display so this is not a correctness issue, but it is inconsistent with the surrounding pattern. Wrap the rename call with .with_context(|| format!("failed to rename: {filename}")) for consistency.
