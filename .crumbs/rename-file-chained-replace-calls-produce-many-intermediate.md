---
id: meta-xdh
title: 'rename_file: chained .replace() calls produce many intermediate Strings'
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

# rename_file: chained .replace() calls produce many intermediate Strings

F-11: Lines 38-54 chain up to nine .replace() calls, each producing a new String. A single-pass approach (e.g. chars().fold or a manual replace loop) would reduce allocations. Not a hot path but a good hygiene fix.
