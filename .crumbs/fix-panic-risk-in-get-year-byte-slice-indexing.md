---
id: meta-m36
title: Fix panic risk in get_year byte-slice indexing
status: open
type: bug
priority: 1
tags:
- utils
- panic
created: 2026-04-03
updated: 2026-04-03
dependencies: []
---

# Fix panic risk in get_year byte-slice indexing

utils.rs:24 uses `subdate[0..4]` (byte-index slice) on a string that may be shorter than 4 bytes (e.g. if the "D:" split yields an empty second segment). Should use `subdate.get(0..4).unwrap_or("")` or chars-based slicing to avoid a runtime panic.
