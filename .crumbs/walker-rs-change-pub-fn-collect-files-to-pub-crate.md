---
id: meta-8z5
title: 'walker.rs: change pub fn collect_files to pub(crate)'
status: open
type: task
priority: 3
tags:
- walker
- api
created: 2026-04-04
updated: 2026-04-04
dependencies: []
---

# walker.rs: change pub fn collect_files to pub(crate)

F-16: collect_files is declared pub inside mod walker (a private module). The pub is unreachable by external crates and misleading. Use pub(crate) to signal crate-internal visibility, or expose via lib.rs if a library target is ever added.
