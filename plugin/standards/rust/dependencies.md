---
name: Managing Rust Dependencies
description: Guidelines for selecting and managing third-party Rust crates using blessed.rs, lib.rs, and GitHub metrics
when_to_use: when adding third-party dependencies to Rust projects to ensure quality and maintainability
applies_to: Rust
version: 1.0.0
---

# Managing Dependencies

## Don't reinvent the wheel

Use third-party libraries wherever possible.


## Selecting third-party libraries
When adding third-party dependencies:
- Select the most popular and actively maintained option
- When selecting Rust libraries or crates:
  - Check library on https://blessed.rs/ (anything listed is an idiomatic choice)
  - Check the library https://lib.rs/ for:
     - Number of downloads per month
     - Recent releases (within last 6 months)
- Check the library's GitHub repository for:
  - Recent commits (within last 6 months)
  - Active issue resolution
  - Number of stars/downloads
  - Clear documentation
