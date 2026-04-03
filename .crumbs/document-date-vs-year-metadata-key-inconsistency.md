---
id: meta-kne
title: Document Date vs Year metadata key inconsistency
status: open
type: task
priority: 3
tags:
- docs
- epub
- pdf
created: 2026-04-03
updated: 2026-04-03
dependencies: []
---

# Document Date vs Year metadata key inconsistency

PDF extraction inserts "Year" directly; EPUB and MOBI insert "Date" which main.rs converts to "Year" via get_year(). This implicit contract is undocumented. Add a code comment in main.rs explaining the normalisation step, and consider whether get_year() should live in epub.rs/mobi.rs or remain in main.rs.
