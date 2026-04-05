---
id: meta-cfz
title: 'rename_file: to_string_lossy() called multiple times on same path'
status: closed
type: task
priority: 3
tags:
- rename_file
- allocations
created: 2026-04-04
updated: 2026-04-05
dependencies: []
---

# rename_file: to_string_lossy() called multiple times on same path

F-12: new_path.to_string_lossy() is called at lines 79, 81, 99, 103-104, and 111. Binding the result once (let lossy = new_path.to_string_lossy()) and reusing it avoids repeated Cow construction and makes intent clearer.
