---
name: dual-verification-review
description: Use two independent agents to systematically verify content against ground truth, then collate findings to identify common issues, exclusive issues, and divergences
when_to_use: comprehensive documentation audits, plan reviews, code reviews, verifying content matches implementation, quality assurance for critical content
version: 1.0.0
---

# Dual Verification Review

## Overview

Use two independent agents to systematically verify content against ground truth, then use a collation agent to compare findings.

**Core principle:** Independent dual review + systematic collation = higher quality, managed context.

**Announce at start:** "I'm using the dual-verification-review skill for comprehensive review."

## When to Use

Use dual-verification-review when:

- **High-stakes decisions:** Before executing implementation plans, merging to production, or deploying
- **Comprehensive audits:** Documentation accuracy, plan quality, code correctness
- **Quality assurance:** Critical content that must be verified against ground truth
- **Risk mitigation:** When cost of missing issues exceeds cost of dual review
- **Building confidence:** When you need high-confidence assessment before proceeding

**Don't use when:**
- Simple, low-stakes changes (typo fixes, minor documentation tweaks)
- Time-critical situations (production incidents requiring immediate action)
- Single perspective is sufficient (trivial updates, following up on previous review)
- Cost outweighs benefit (incremental updates to well-tested code)

## Quick Reference

| Phase | Action | Output |
|-------|--------|--------|
| **Phase 1** | Dispatch 2 agents in parallel with identical prompts | Two independent review reports |
| **Phase 2** | Dispatch collation agent to compare reviews | Collated report with confidence levels |
| **Phase 3** | Present findings to user | Common (fix now), Exclusive (decide), Divergences (investigate) |

**Confidence levels:**
- **VERY HIGH:** Both reviewers found (fix immediately)
- **MODERATE:** One reviewer found (requires judgment)
- **INVESTIGATE:** Reviewers disagree (user decides)

## Why This Pattern Works

**Higher quality through independence:**
- Common issues = high confidence (both found)
- Exclusive issues = edge cases one agent caught
- Divergences = areas needing investigation

**Context management:**
- Two detailed reviews = lots of context
- Collation agent does comparison work
- Main context gets clean summary

**Confidence levels:**
- Both found → Very likely real issue → Fix immediately
- One found → Edge case or judgment call → Decide case-by-case
- Disagree → Requires investigation → User makes call

## The Three-Phase Process

### Phase 1: Dual Independent Review

**Dispatch 2 agents in parallel with identical prompts.**

**Agent prompt template:**
```
You are [agent type] conducting an independent verification review.

**Your task:** Systematically verify [subject] against [ground truth].

**Critical instruction:** Current content CANNOT be assumed correct. Verify every claim.

**Process:**

1. Read [subject] completely
2. For each [section/component/claim]:
   - Identify what is claimed
   - Verify against [ground truth]
   - Check for [specific criteria]

3. Identify issues:
   - **[Issue type 1]:** [description]
   - **[Issue type 2]:** [description]
   - **[Issue type 3]:** [description]

4. For each issue, provide:
   - Location ([file/section/line])
   - Current content (what [subject] says)
   - Actual [ground truth] (what is true)
   - Severity ([severity levels])

**Output format:**

# [Agent Type] Review Report

## [Section Name] Issues

### [Specific Location]
- **Issue:** [description]
- **Current content:** [what is claimed]
- **Actual [ground truth]:** [what is true]
- **Severity:** [critical/high/medium/low]

## Summary
- Total issues found: X
- [Severity breakdown]
```

**Example: Documentation Review**
- Agent type: technical-writer
- Subject: README.md and CLAUDE.md
- Ground truth: current codebase implementation
- Criteria: file paths exist, commands work, examples accurate

**Example: Plan Review**
- Agent type: plan-reviewer
- Subject: implementation plan
- Ground truth: 35 quality criteria (security, testing, architecture, etc.)
- Criteria: blocking issues, non-blocking improvements

**Example: Code Review**
- Agent type: code-reviewer
- Subject: implementation code
- Ground truth: coding standards, plan requirements
- Criteria: meets requirements, follows standards, has tests

### Phase 2: Collate Findings

**Dispatch collation agent to compare the two reviews.**

**Collation agent prompt template:**
```
You are collating two independent [review type] reviews.

**Inputs:**
- Review #1: [full review from agent 1]
- Review #2: [full review from agent 2]

**Your task:**

1. **Identify common issues** (both found):
   - High confidence these are real issues
   - List with: issue description, location, severity, both agents' findings

2. **Identify exclusive issues** (only one found):
   - Agent #1 only: [list issues]
   - Agent #2 only: [list issues]
   - Note: These may be edge cases or require judgment

3. **Identify divergences** (agents disagree or contradict):
   - List issue where agents have different conclusions
   - Show both perspectives
   - Note: These require investigation

4. **Assess overall confidence:**
   - Issues found by both = very high confidence
   - Issues found by one = moderate confidence (depends on reasoning)
   - Contradictions = requires user decision

**Output format:**

# Collated Review Report

## Common Issues (High Confidence)
Both reviewers independently found these issues.

### [Severity Level]
- **[Issue]** ([Location])
  - Agent #1 finding: [description]
  - Agent #2 finding: [description]
  - Confidence: VERY HIGH

## Exclusive Issues (Requires Judgment)
Only one reviewer found these issues.

### Found by Agent #1 Only
- **[Issue]** ([Location])
  - Finding: [description]
  - Severity: [level]
  - Confidence: MODERATE

### Found by Agent #2 Only
- **[Issue]** ([Location])
  - Finding: [description]
  - Severity: [level]
  - Confidence: MODERATE

## Divergences (Requires Investigation)
Reviewers disagree or have contradictory findings.

- **[Issue]** ([Location])
  - Agent #1 says: [perspective]
  - Agent #2 says: [different perspective]
  - Confidence: INVESTIGATE

## Summary
- Total unique issues: X
- Common (high confidence): X
- Exclusive (judgment): X
- Divergences (investigate): X

## Recommendation
[Overall assessment and suggested actions]
```

### Phase 3: Present Findings to User

**Present collated report with clear action items:**

1. **Common issues** (both found):
   - These should be addressed immediately
   - Very high confidence they're real problems

2. **Exclusive issues** (one found):
   - User decides case-by-case
   - Review agent's reasoning
   - May be edge cases or may be missed by other agent

3. **Divergences** (agents disagree):
   - User investigates and makes final call
   - May need additional verification
   - May indicate ambiguity in requirements/standards

## Parameterization

Make the pattern flexible by specifying:

**Subject:** What to review
- Documentation files (README.md, CLAUDE.md)
- Implementation plans (plan.md)
- Code changes (git diff, specific files)
- Test coverage (test files)
- Architecture decisions (design docs)

**Ground truth:** What to verify against
- Current implementation (codebase)
- Quality criteria (35-point checklist)
- Coding standards (practices)
- Requirements (specifications)
- Design documents (architecture)

**Agent type:** Which specialized agent to use
- technical-writer (documentation)
- plan-reviewer (plans)
- code-reviewer (code)
- rust-engineer (Rust-specific)
- ultrathink-debugger (complex issues)

**Granularity:** How to break down review
- Section-by-section (documentation)
- Criteria-by-criteria (plan review)
- File-by-file (code review)
- Feature-by-feature (architecture review)

**Severity levels:** How to categorize issues
- critical/high/medium/low (general)
- BLOCKING/NON-BLOCKING (plan/code review)
- security/performance/maintainability (code review)

## When NOT to Use

**Skip dual verification when:**
- Simple, low-stakes changes (typo fixes)
- Time-critical situations (production incidents)
- Single perspective sufficient (trivial updates)
- Cost outweighs benefit (minor documentation tweaks)

**Use single agent when:**
- Regular incremental updates
- Following up on dual review findings
- Implementing approved changes

## Example Usage: Plan Review

```
User: Review this implementation plan before execution

You: I'm using the dual-verification-review skill for comprehensive review.

Phase 1: Dual Independent Review
  → Dispatch 2 plan-reviewer agents in parallel
  → Each applies 35 quality criteria independently
  → Agent #1 finds: 3 BLOCKING issues, 7 NON-BLOCKING
  → Agent #2 finds: 4 BLOCKING issues, 5 NON-BLOCKING

Phase 2: Collate Findings
  → Dispatch general-purpose agent as collator
  → Collator compares both reviews
  → Produces collated report

Collated Report:
  Common Issues (High Confidence):
    - 2 BLOCKING issues both found
    - 3 NON-BLOCKING issues both found

  Exclusive Issues:
    - Agent #1 only: 1 BLOCKING, 4 NON-BLOCKING
    - Agent #2 only: 2 BLOCKING, 2 NON-BLOCKING

  Divergences: None

Phase 3: Present to User
  → Show common BLOCKING issues (fix immediately)
  → Show exclusive BLOCKING issues (user decides)
  → Show all NON-BLOCKING for consideration
```

## Example Usage: Documentation Review

```
User: Audit README.md and CLAUDE.md for accuracy

You: I'm using the dual-verification-review skill for comprehensive documentation audit.

Phase 1: Dual Independent Review
  → Dispatch 2 technical-writer agents in parallel
  → Each verifies docs against codebase
  → Agent #1 finds: 13 issues (1 critical, 3 high, 6 medium, 3 low)
  → Agent #2 finds: 13 issues (4 critical, 1 high, 4 medium, 4 low)

Phase 2: Collate Findings
  → Dispatch general-purpose agent as collator
  → Identifies: 7 common, 6 exclusive, 0 divergences

Collated Report:
  Common Issues (High Confidence): 7
    - Missing mise commands (CRITICAL)
    - Incorrect skill path (MEDIUM)
    - Missing /plan-review command (HIGH)

  Exclusive Issues: 6
    - Agent #1 only: 3 issues
    - Agent #2 only: 3 issues

Phase 3: Present to User
  → Fix common issues immediately (high confidence)
  → User decides on exclusive issues case-by-case
```

## Related Skills

**When to use this skill:**
- Comprehensive reviews before major actions
- High-stakes decisions (execution, deployment, merge)
- Quality assurance for critical content

**Other review skills:**
- conducting-plan-review: Single plan-reviewer (faster, less thorough)
- conducting-code-review: Single code-reviewer (regular reviews)
- maintaining-docs-after-changes: Single technical-writer (incremental updates)

**Use dual-verification-review when stakes are high, use single-agent skills for regular work.**

## Common Mistakes

**Mistake:** "The reviews mostly agree, I'll skip detailed collation"
- **Why wrong:** Exclusive issues and subtle divergences matter
- **Fix:** Always use collation agent for systematic comparison

**Mistake:** "This exclusive issue is probably wrong since other reviewer didn't find it"
- **Why wrong:** May be valid edge case one reviewer caught
- **Fix:** Present with MODERATE confidence for user judgment, don't dismiss

**Mistake:** "I'll combine both reviews myself instead of using collation agent"
- **Why wrong:** Context overload, missing patterns, inconsistent categorization
- **Fix:** Always dispatch collation agent to handle comparison work

**Mistake:** "Two agents is overkill, I'll just run one detailed review"
- **Why wrong:** Missing the independence that catches different perspectives
- **Fix:** Use dual verification for high-stakes, single review for regular work

**Mistake:** "The divergence is minor, I'll pick one perspective"
- **Why wrong:** User needs to see both perspectives and make informed decision
- **Fix:** Mark as INVESTIGATE and let user decide

## Remember

- Dispatch 2 agents in parallel for Phase 1 (efficiency)
- Use identical prompts for both agents (fairness)
- Dispatch collation agent for Phase 2 (context management)
- Present clean summary to user in Phase 3 (usability)
- Common issues = high confidence (both found)
- Exclusive issues = requires judgment (one found)
- Divergences = investigate (agents disagree)
- Cost-benefit: Use for high-stakes, skip for trivial changes