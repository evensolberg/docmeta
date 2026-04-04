---
id: meta-wvv
title: 'rename_file: trim().to_string() always clones the string'
status: open
type: task
priority: 3
tags:
- rename_file
- allocations
created: 2026-04-04
updated: 2026-04-04
dependencies: []
---

# rename_file: trim().to_string() always clones the string

F-10: Line 61 does new_filename = new_filename.trim().to_string(). trim() borrows into the existing String; .to_string() then allocates a new one. When there is nothing to trim (common case) this is a pure waste. Consider in-place trimming or accepting the cost with a comment.
