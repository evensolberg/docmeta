---
id: meta-clf
title: 'main.rs: replace empty_str workaround with .map(String::as_str).unwrap_or("")'
status: open
type: task
priority: 3
tags:
- main
- cli
created: 2026-04-04
updated: 2026-04-04
dependencies: []
---

# main.rs: replace empty_str workaround with .map(String::as_str).unwrap_or("")

F-17: Lines 98-101 create a local String::new() solely to produce a &str fallback for get_one::<String>. Replace with .get_one::<String>("rename-pattern").map(String::as_str).unwrap_or("") or add .default_value("") to the clap arg definition.
