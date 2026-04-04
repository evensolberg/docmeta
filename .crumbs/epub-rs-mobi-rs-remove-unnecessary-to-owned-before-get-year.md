---
id: meta-ueo
title: 'epub.rs / mobi.rs: remove unnecessary .to_owned() before get_year call'
status: open
type: task
priority: 3
tags:
- epub
- mobi
- allocations
created: 2026-04-04
updated: 2026-04-04
dependencies: []
---

# epub.rs / mobi.rs: remove unnecessary .to_owned() before get_year call

F-07: .map_or("", String::as_str).to_owned() allocates a String that is immediately borrowed as &str by get_year. Bind as &str directly: let date: &str = metadata_map.get("Date").map_or("", String::as_str); — no to_owned() needed.
