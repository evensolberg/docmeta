---
id: meta-vz8
title: Remove ignore from utils.rs doctests; fix epub.rs doctest
status: open
type: task
priority: 3
tags:
- docs
- testing
created: 2026-04-04
updated: 2026-04-04
dependencies: []
---

# Remove ignore from utils.rs doctests; fix epub.rs doctest

F-19: get_extension and get_year examples in utils.rs have no I/O dependency — remove the ignore marker and let them run as real doctests. epub.rs get_metadata example requires a fixture path; use no_run if I/O is unavoidable, or provide a preamble that sets up the path.
