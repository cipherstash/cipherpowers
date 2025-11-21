# Plan Review

Evaluate implementation plans before execution to ensure they're comprehensive, executable, and account for all quality criteria.

**Default mode:** Dual-verification review for comprehensive quality assurance.

## Algorithmic Workflow

**Decision tree (follow exactly, no interpretation):**

1. Is this a plan review request?
   - YES → Continue to step 2
   - NO → This command was invoked incorrectly

2. Is there a plan file to review?
   - YES → Continue to step 3
   - NO → Ask user for plan file location
   - UNSURE → Check `.work/` directory for recent plan files

3. **USE DUAL-VERIFICATION-REVIEW SKILL:**

```
Announce: "I'm using the dual-verification-review skill for comprehensive plan review."

Use Skill tool with:
  skill: "cipherpowers:dual-verification-review"
```

4. **FOLLOW THE DUAL-VERIFICATION PATTERN:**

**Phase 1: Dispatch 2 plan-reviewer agents (in parallel):**

```
Agent #1:
  subagent_type: "cipherpowers:plan-reviewer"
  description: "Plan review - Agent #1"
  prompt: "You are plan-reviewer agent #1 conducting independent plan evaluation.

  Review the implementation plan at [path].

  Follow conducting-plan-review skill to evaluate against all 35 quality criteria.

  Report all BLOCKING and NON-BLOCKING issues found."

Agent #2:
  subagent_type: "cipherpowers:plan-reviewer"
  description: "Plan review - Agent #2"
  prompt: "You are plan-reviewer agent #2 conducting independent plan evaluation.

  Review the implementation plan at [path].

  Follow conducting-plan-review skill to evaluate against all 35 quality criteria.

  Report all BLOCKING and NON-BLOCKING issues found."
```

**Phase 2: Dispatch collation agent:**

```
Use Task tool with:
  subagent_type: "cipherpowers:review-collator"
  description: "Collate plan review findings"
  prompt: "Collate these two independent plan reviews:

  Review #1: [agent #1 full output]
  Review #2: [agent #2 full output]

  Systematically identify:
  - Common issues (high confidence)
  - Exclusive issues (requires judgment)
  - Divergences (requires investigation)

  Follow review-collator agent workflow to produce structured collated report."
```

**Phase 3: Present findings to user:**
- Common BLOCKING issues (fix immediately)
- Exclusive BLOCKING issues (user decides)
- NON-BLOCKING suggestions (for consideration)
- Divergences (investigate)

5. **STOP when collation completes and findings presented.**

## Single Agent Mode (Fallback)

**For time-sensitive reviews or follow-up reviews:**

User can request `--single` mode:

```
Use Task tool with:
  subagent_type: "cipherpowers:plan-reviewer"
  prompt: "Review the implementation plan at [path]. Follow the conducting-plan-review skill to evaluate against all quality criteria."
```

## Why Algorithmic Workflow?

- **100% reliability**: No interpretation, no skipping checklist categories
- **Dual verification**: Two independent reviews catch more issues
- **Confidence levels**: Common issues = high confidence, exclusive = judgment
- **Context management**: Collation agent manages comparison, clean summary to user
- **Quality gate**: Catches plan issues before expensive implementation

## Why Dual Verification by Default?

**Plans have high stakes:**
- Wrong plan = wasted implementation time + rework + bugs
- Catching issues pre-execution is 10x cheaper than post-execution

**Multiple perspectives find more issues:**
- Security perspective catches auth/data risks
- Performance perspective catches scaling issues
- Architecture perspective catches design issues
- Common issues = very high confidence they're real problems

**Cost-benefit:**
- 2x agents + 1x collation = 3x cost
- But catches issues before expensive implementation
- ROI positive for any non-trivial plan

## What the Dual Verification Does

**Phase 1: Independent dual review**
- 2 plan-reviewer agents apply 35 quality criteria independently
- Each evaluates: Security, Testing, Architecture, Error Handling, Code Quality, Process
- Each assesses plan structure (granularity, completeness, TDD)

**Phase 2: Systematic collation**
- Collation agent compares findings
- Identifies: Common issues (high confidence), Exclusive issues (judgment), Divergences (investigate)
- Confidence levels guide user decisions

**Phase 3: Clean user presentation**
- Common BLOCKING issues → Fix immediately
- Exclusive BLOCKING issues → User decides case-by-case
- NON-BLOCKING suggestions → Consider for quality improvement
- Divergences → Investigate and make final call

**References:**
- Skill: `${CLAUDE_PLUGIN_ROOT}skills/dual-verification-review/SKILL.md`
- Plan Review Skill: `${CLAUDE_PLUGIN_ROOT}skills/conducting-plan-review/SKILL.md`
- Plan Reviewer Agent: `${CLAUDE_PLUGIN_ROOT}agents/plan-reviewer.md`
- Review Collator Agent: `${CLAUDE_PLUGIN_ROOT}agents/review-collator.md`
- Template: `${CLAUDE_PLUGIN_ROOT}templates/plan-evaluation-template.md`
- Integration: Use before `/execute` to validate plans
