---
id: meta-8iv
title: 'get_year: eliminate double allocation in non-early-return path'
status: open
type: task
priority: 3
tags:
- utils
- allocations
created: 2026-04-04
updated: 2026-04-04
dependencies: []
---

# get_year: eliminate double allocation in non-early-return path

F-09: The function allocates a String into `year` then calls year.trim().to_string() allocating again. Replace with a single chain: date.split("-").next().unwrap_or("").trim().to_string() — one allocation instead of two.
