---
id: meta-hoz
title: Add .crumbs/index.csv to .gitignore
status: closed
type: task
priority: 1
tags:
- infra
- crumbs
created: 2026-04-03
updated: 2026-04-03
closed_reason: Added .crumbs/index.csv to .gitignore
dependencies: []
---

# Add .crumbs/index.csv to .gitignore

The .crumbs/ directory is tracked by git but index.csv is regenerated on every crumbs write and should not be committed. Add `.crumbs/index.csv` to .gitignore.
