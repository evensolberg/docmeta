---
id: meta-e0u
title: Add rustdoc to pdf, mobi, utils, and cli modules
status: closed
type: task
priority: 2
tags:
- docs
created: 2026-04-03
updated: 2026-04-03
dependencies: []
---

# Add rustdoc to pdf, mobi, utils, and cli modules

Only epub.rs has comprehensive rustdoc comments. Add module-level and function-level doc comments to: pdf::get_metadata, mobi::get_metadata, utils::print_metadata, utils::new_hashmap, cli::build. Also fix the misplaced `///` comment on the test module in utils.rs (line 52).
