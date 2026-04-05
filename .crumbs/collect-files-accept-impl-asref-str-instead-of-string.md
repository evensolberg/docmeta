---
id: meta-m83
title: 'collect_files: accept &[impl AsRef<str>] instead of &[String]'
status: closed
type: feature
priority: 2
tags:
- walker
- api
- allocations
created: 2026-04-04
updated: 2026-04-04
dependencies: []
---

# collect_files: accept &[impl AsRef<str>] instead of &[String]

F-02/F-03: The function only reads inputs as &str but forces callers to own Vec<String>. Change signature to inputs: &[impl AsRef<str>] and use input.as_ref() internally. Eliminates the .cloned().collect::<Vec<_>>() in main.rs (lines 51-54) that exists solely to satisfy this signature.
