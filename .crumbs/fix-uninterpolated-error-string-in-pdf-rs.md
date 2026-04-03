---
id: meta-850
title: Fix uninterpolated error string in pdf.rs
status: closed
type: bug
priority: 1
tags:
- pdf
- error-handling
created: 2026-04-03
updated: 2026-04-03
closed_reason: 'Fixed: changed string literal to format\!() so filename is interpolated; covered by pdf::tests::error_message_contains_filename_when_no_info_dict'
dependencies: []
---

# Fix uninterpolated error string in pdf.rs

Line 22: `return Err("No info dictionary found in {filename}".into())` is a string literal — `{filename}` is never substituted. Should be `format\!("No info dictionary found in {filename}").into()`.
