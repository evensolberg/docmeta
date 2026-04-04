---
id: meta-akx
title: 'Metadata HashMap: replace "Unknown" sentinel with Option<String>'
status: open
type: feature
priority: 2
tags:
- api
- epub
- mobi
- pdf
created: 2026-04-04
updated: 2026-04-04
dependencies: []
---

# Metadata HashMap: replace "Unknown" sentinel with Option<String>

F-14: HashMap<String, String> conflates absent fields ("Unknown" literal in pdf.rs, empty string elsewhere) with present ones. Change return type to HashMap<String, Option<String>> or introduce a typed Metadata struct with Option<String> fields so callers can distinguish absent vs unknown.
