---
id: meta-idh
title: 'get_field! macro: replace clone with a free function'
status: closed
type: task
priority: 2
tags:
- pdf
- macro
- allocations
created: 2026-04-04
updated: 2026-04-04
dependencies: []
---

# get_field! macro: replace clone with a free function

F-13: The macro clones Option<PdfString> to avoid a borrow issue. Extract a fn pdf_string_to_string(s: Option<&PdfString>) -> String instead. Removes the clone, makes the logic testable in isolation, and decouples the macro from concrete field names.
