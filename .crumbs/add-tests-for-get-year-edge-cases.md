---
id: meta-hbx
title: Add tests for get_year edge cases
status: closed
type: task
priority: 2
tags:
- testing
- utils
created: 2026-04-03
updated: 2026-04-03
closed_reason: 'Done: test_get_year_edge_cases added in PR #13, also covers D: timezone colon notation'
dependencies: []
---

# Add tests for get_year edge cases

The existing test_get_year covers happy paths. Add edge-case tests: empty string input, "D:" with no following characters (triggers the current panic risk), strings shorter than 4 chars after splitting, non-numeric year segments. These tests will also validate the panic-fix (meta-m36) once implemented.
