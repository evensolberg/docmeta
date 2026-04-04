---
id: meta-ln4
title: Fix stale doc comment in rename_file.rs
status: closed
type: bug
priority: 2
tags:
- docs
- rename
created: 2026-04-03
updated: 2026-04-03
dependencies: []
---

# Fix stale doc comment in rename_file.rs

The rustdoc for `rename_file()` lists a `config: &DefaultValues` parameter that does not exist in the actual function signature (the function takes `dry_run: bool` instead). This misleads callers. Update the doc comment to match the real signature.
