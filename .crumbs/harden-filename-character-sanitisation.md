---
id: meta-3tc
title: Harden filename character sanitisation
status: open
type: idea
priority: 3
tags:
- rename
created: 2026-04-03
updated: 2026-04-03
dependencies: []
---

# Harden filename character sanitisation

rename_file.rs only strips `/`, `:`, and `.` from generated filenames. macOS additionally forbids NUL; Windows forbids `\ * ? " < > |` and reserved names (CON, PRN, etc.). Consider: (a) a platform-specific forbidden-char list, or (b) using a crate like `sanitize-filename` to handle cross-platform concerns.
