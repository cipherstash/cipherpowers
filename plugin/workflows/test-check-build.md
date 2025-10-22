---
name: Test-Check-Build Workflow
description: Essential quality gates ensuring tests pass, checks pass, and code builds successfully
when_to_use: before committing code, before completing work in execute workflow, or any time code quality verification is needed
related_practices: testing.md, git-guidelines.md
version: 1.0.0
---

# Test-Check-Build

## 1. Run tests

```bash
mise run test
```

## 2. Run checks

```bash
mise run check
```

## 3. Build

```bash
mise run build
```

## 4. Verify all checks passed

Do all tests, checks, and build pass?
