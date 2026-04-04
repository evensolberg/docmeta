---
id: meta-jxn
title: 'epub.rs: key.to_string().to_case() — remove intermediate allocation'
status: open
type: task
priority: 3
tags:
- epub
- allocations
created: 2026-04-04
updated: 2026-04-04
dependencies: []
---

# epub.rs: key.to_string().to_case() — remove intermediate allocation

F-06: Lines 69 and 72 call key.to_string().to_case(Case::Title). The convert_case Casing trait is implemented on &str directly, so key.to_case(Case::Title) produces the result in one step. Remove the .to_string() intermediate.
