---
id: meta-9c7
title: Fix epub.rs breakage from epub crate 2.1.5 API change
status: closed
type: bug
priority: 0
tags:
- epub
- dependencies
created: 2026-04-03
updated: 2026-04-03
closed_reason: 'Fixed: use doc.mdata(key) with the epub 2.1.5 Vec<MetadataItem> API'
dependencies: []
---

# Fix epub.rs breakage from epub crate 2.1.5 API change

Dependabot updated epub 2.1.2 → 2.1.5. The metadata field changed from HashMap<String, Vec<String>> to Vec<MetadataItem>. epub.rs:68 calls .get(key) on a slice (E0277) and line 72 fails type inference (E0282). Fix: use doc.mdata(key) convenience method which returns Option<&MetadataItem>, then access .value on the item.
