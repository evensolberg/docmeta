---
id: meta-77m
title: 'walker tests: replace to_string_lossy().to_string() with into_owned()'
status: open
type: task
priority: 3
tags:
- walker
- testing
- allocations
created: 2026-04-04
updated: 2026-04-04
dependencies: []
---

# walker tests: replace to_string_lossy().to_string() with into_owned()

F-01: Every test calls .to_string_lossy().to_string(), which forces an allocation even on pure-UTF-8 paths (Cow::Borrowed case). Use .into_owned() on the Cow instead — semantically correct and avoids an intermediate Cow<str> -> String step.
