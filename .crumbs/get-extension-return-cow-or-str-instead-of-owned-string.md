---
id: meta-h1i
title: 'get_extension: return Cow or &str instead of owned String'
status: open
type: task
priority: 2
tags:
- utils
- allocations
- api
created: 2026-04-04
updated: 2026-04-04
dependencies: []
---

# get_extension: return Cow or &str instead of owned String

F-08: get_extension returns String but every call site immediately borrows it (&str). Returning Cow<str> or restructuring callers to work with OsStr directly would eliminate per-call allocations. All callers in main.rs and rename_file.rs only need a borrow.
