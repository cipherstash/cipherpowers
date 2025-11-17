---
name: Test-Check-Build Workflow
description: Essential quality gates ensuring tests pass, checks pass, and code builds successfully
when_to_use: before committing code, before completing work in execute workflow, or any time code quality verification is needed
related_practices: testing.md, git-guidelines.md
version: 1.0.0
---

# Test-Check-Build

Consult the project's CLAUDE.md to determine the specific commands for each step.

## 1. Run tests

Run the project's test suite.

- **Purpose:** Verify all unit tests, integration tests, and other automated tests pass
- **Command:** See CLAUDE.md for the specific test command for this project
- **Requirement:** ALL tests MUST pass

## 2. Run checks

Run the project's quality checks.

- **Purpose:** Verify linting, formatting, type checking, and other static analysis
- **Command:** See CLAUDE.md for the specific check command for this project
- **Requirement:** ALL checks MUST pass

## 3. Build

Build/compile the project.

- **Purpose:** Verify code compiles successfully and can be built
- **Command:** See CLAUDE.md for the specific build command for this project
- **Requirement:** Build MUST succeed without errors

## 4. Verify all checks passed

Do all tests, checks, and build pass?

- **YES:** Quality gates passed - code is ready
- **NO:** Fix failing checks before proceeding
