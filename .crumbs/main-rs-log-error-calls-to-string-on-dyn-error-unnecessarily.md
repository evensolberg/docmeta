---
id: meta-3jp
title: 'main.rs: log::error! calls .to_string() on dyn Error unnecessarily'
status: open
type: task
priority: 3
tags:
- main
- allocations
created: 2026-04-04
updated: 2026-04-04
dependencies: []
---

# main.rs: log::error! calls .to_string() on dyn Error unnecessarily

F-05: Line 119 uses log::error!("{}", err.to_string().replace(...)) — Display for dyn Error already calls to_string implicitly. Use log::error!("{err}") and address embedded quotes at the error source rather than at the display site.
