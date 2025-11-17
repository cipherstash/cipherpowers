---
name: Test-Check-Build Workflow
description: Essential quality gates ensuring tests pass, checks pass, and code builds successfully
when_to_use: before committing code, before completing work in execute workflow, or any time code quality verification is needed
related_practices: testing.md, git-guidelines.md
version: 1.0.0
---

# Test-Check-Build

## 1. Run tests

Run project test command

- **Requirement:** ALL tests MUST pass

## 2. Run checks

Run project check command

- **Requirement:** ALL checks MUST pass

## 3. Build

Run project build command

- **Requirement:** Build MUST succeed without errors

## 4. Verify all checks passed

Do all tests, checks, and build pass?

- **YES:** Quality gates passed - code is ready
- **NO:** Fix failing checks before proceeding
