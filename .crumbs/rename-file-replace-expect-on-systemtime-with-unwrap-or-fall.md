---
id: meta-hwc
title: 'rename_file: replace expect() on SystemTime with unwrap_or fallback'
status: open
type: bug
priority: 3
tags:
- rename_file
- panic
created: 2026-04-04
updated: 2026-04-04
dependencies: []
---

# rename_file: replace expect() on SystemTime with unwrap_or fallback

F-20: Line 121 uses .expect("Time went backwards...") on SystemTime::now().duration_since(UNIX_EPOCH). The panic is reachable (clock before epoch, sandboxed env). Replace with .unwrap_or(Duration::ZERO).as_micros() to return 0 gracefully instead of panicking.
