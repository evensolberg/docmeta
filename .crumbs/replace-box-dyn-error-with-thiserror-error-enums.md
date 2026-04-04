---
id: meta-07h
title: Replace Box<dyn Error> with thiserror error enums
status: open
type: feature
priority: 2
tags:
- error-handling
- api
created: 2026-04-04
updated: 2026-04-04
dependencies: []
---

# Replace Box<dyn Error> with thiserror error enums

F-04/F-18: Every public function returning Result uses Box<dyn Error>, erasing type info and costing a heap allocation per error. Add thiserror to [dependencies] and define per-module error enums. anyhow suits the binary run() fn; thiserror suits epub/mobi/pdf/rename module errors.
