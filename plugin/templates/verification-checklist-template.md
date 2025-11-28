---
name: Verification Checklist Template
description: Quality gate checklist with clear pass/fail criteria and escalation procedures
when_to_use: when creating pre-merge checklists, quality gates, verification procedures, or workflow boundary checks
---

# [Workflow Stage] Verification Checklist

**Purpose:** [What this checklist validates]
**When to use:** [At what point in the workflow]
**Last Updated:** [YYYY-MM-DD]

---

## Pre-Requisites

Before starting this verification:

- [ ] [Prerequisite 1 - e.g., "All changes committed"]
- [ ] [Prerequisite 2 - e.g., "Feature branch up to date with main"]

---

## Automated Checks (MUST PASS)

These checks must all pass. **Do not proceed if any fail.**

### [Check Category 1: e.g., Tests]

- [ ] **Unit tests pass**
  ```bash
  [exact command to run]
  ```
  Expected: All tests green

- [ ] **Integration tests pass**
  ```bash
  [exact command to run]
  ```
  Expected: All tests green

### [Check Category 2: e.g., Linting]

- [ ] **Linter passes**
  ```bash
  [exact command]
  ```
  Expected: No errors

- [ ] **Type check passes**
  ```bash
  [exact command]
  ```
  Expected: No type errors

---

## Manual Verification (SHOULD REVIEW)

Review these items. Use judgment on whether issues are blocking.

### [Category: e.g., Code Quality]

- [ ] **No magic numbers** - All constants named and documented
- [ ] **Error handling complete** - All error paths handled appropriately
- [ ] **Logging adequate** - Key operations logged for debugging

### [Category: e.g., Documentation]

- [ ] **Public APIs documented** - All public functions have doc comments
- [ ] **README updated** - If behavior changed, README reflects it
- [ ] **Breaking changes noted** - CHANGELOG updated if applicable

### [Category: e.g., Security]

- [ ] **No secrets in code** - API keys, passwords externalized
- [ ] **Input validation present** - User input sanitized
- [ ] **Dependencies audited** - No known vulnerabilities

---

## If Automated Checks Fail

**STOP. Do not proceed.**

1. **Read the error message carefully**
   - What test/check failed?
   - What was the expected vs actual behavior?

2. **Check recent changes**
   - Did a recent commit introduce the failure?
   - Can you isolate the breaking change?

3. **Fix the issue**
   - Make the minimal fix necessary
   - Re-run the full check suite

4. **If stuck after [X] minutes**
   - Ask for help
   - Don't force through failing checks

---

## If Manual Checks Have Issues

For each issue found:

| Severity | Action |
|----------|--------|
| **Blocking** | Must fix before proceeding |
| **Non-blocking** | Create follow-up task, note in PR |
| **Discussion needed** | Flag for team review |

---

## Sign-Off

**Verification completed by:** [Name/Date]

| Category | Status | Notes |
|----------|--------|-------|
| Automated checks | ✅ / ❌ | |
| Manual review | ✅ / ⚠️ | |
| Ready to proceed | YES / NO | |

---

## Related

- **Previous stage:** [Link to prior workflow stage]
- **Next stage:** [Link to next workflow stage]
- **Escalation:** [Who to contact if blocked]
