---
name: Code Review Template
description: Structured format for saving code review feedback with severity-based categorization
when_to_use: when conducting code reviews and saving structured feedback to work directories
related_practices: code-review.md
version: 1.0.0
---

# Code Review - {Date}

## Status: [BLOCKED | APPROVED WITH NON-BLOCKING SUGGESTIONS | APPROVED]

<!--
Status guidance:
- BLOCKED: Has BLOCKING issues that must be fixed before merge
- APPROVED WITH NON-BLOCKING SUGGESTIONS: Ready to merge, but consider addressing suggestions
- APPROVED: Clean, ready to merge with no issues
-->


## Test Results
- Status: [PASS/FAIL]
- Details: [test failures or "all passed"]


## Check Results
- Status: [PASS/FAIL]
- Details: [warnings or "clean"]


## Next Steps
[Actions required]


## BLOCKING (Must Fix Before Merge)

[Issues or "None"]

**[Issue title]:**
- Description: [clear description]
- Location: [file:line]
- Action: [specific, actionable suggestion]

**[Issue title]:**
- Description: [clear description]
- Location: [file:line]
- Action: [specific, actionable suggestion]


## NON-BLOCKING (May Be Deferred)

[Issues or "None"]

**[Issue title]:**
- Description: [clear description]
- Location: [file:line]
- Action: [specific & actionable suggestion]

**[Issue title]:**
- Description: [clear description]
- Location: [file:line]
- Action: [specific & actionable suggestion]


## Highlights

[Concise reference to specific examples: clear abstractions, effective tests, thoughtful design, clear naming]

**[Example]:**
- What: [what makes this good]
- Location: [file:line]

**[Example]:**
- What: [what makes this good]
- Location: [file:line]

