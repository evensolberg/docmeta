---
id: meta-kez
title: Add recursive directory traversal option
status: closed
type: idea
priority: 3
tags:
- feature
- cli
created: 2026-04-03
updated: 2026-04-04
dependencies: []
---

# Add recursive directory traversal option

Currently only glob/wildcard expansion by the shell is supported. Add a `-R / --recursive` flag that walks a directory tree and processes all ebook files found. Useful for bulk library organisation. The `walkdir` crate is a natural fit.
