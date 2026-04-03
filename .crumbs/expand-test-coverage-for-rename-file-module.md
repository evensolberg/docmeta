---
id: meta-15d
title: Expand test coverage for rename_file module
status: open
type: task
priority: 2
tags:
- testing
- rename
created: 2026-04-03
updated: 2026-04-03
dependencies: []
---

# Expand test coverage for rename_file module

rename_file.rs has zero tests despite being the most complex module. Add unit tests for: pattern substitution (%t/%a/%p/%i/%y), empty-pattern error, all-empty-metadata fallback to "Unknown", slash/colon/period sanitisation in metadata values, collision handling (unique suffix added when file exists), dry_run=true skips actual fs::rename.
