---
id: meta-uq1
title: 'Add #[must_use] to all public Result-returning functions'
status: open
type: task
priority: 3
tags:
- api
- correctness
created: 2026-04-04
updated: 2026-04-04
dependencies: []
---

# Add #[must_use] to all public Result-returning functions

F-15: No public function is annotated with #[must_use]. Functions returning Result (get_metadata in epub/mobi/pdf, rename_file, collect_files, get_extension) should carry #[must_use] so silently discarding results is a compile-time warning.
