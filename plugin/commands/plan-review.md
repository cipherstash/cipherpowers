# Plan Review

Evaluate implementation plans before execution to ensure they're comprehensive, executable, and account for all quality criteria.

## Algorithmic Workflow

**Decision tree (follow exactly, no interpretation):**

1. Is this a plan review request?
   - YES → Continue to step 2
   - NO → This command was invoked incorrectly

2. Is there a plan file to review?
   - YES → Continue to step 3
   - NO → Ask user for plan file location
   - UNSURE → Check `.work/` directory for recent plan files

3. **DISPATCH TO PLAN-REVIEWER AGENT:**

```
Use Task tool with:
  subagent_type: "cipherpowers:plan-reviewer"
  prompt: "Review the implementation plan at [path]. Follow the conducting-plan-review skill to evaluate against all quality criteria (Security & Correctness, Testing, Architecture, Error Handling, Code Quality, Process) and plan structure (task granularity, completeness, TDD approach). Save structured evaluation to .work directory using plan-evaluation-template.md."
```

4. **AGENT WILL:**
   - Follow conducting-plan-review skill
   - Review against all quality categories
   - Evaluate plan structure and completeness
   - Save structured evaluation to `.work/` directory
   - Report findings (BLOCKED | APPROVED WITH SUGGESTIONS | APPROVED)

5. **STOP when agent completes evaluation.**

## Why Algorithmic Workflow?

- **100% reliability**: No interpretation, no skipping checklist categories
- **Skill integration**: Agent uses conducting-plan-review skill
- **Consistent structure**: Every evaluation follows proven template
- **Quality gate**: Catches plan issues before implementation

## What the Agent Does

The plan-reviewer agent provides:
- Systematic evaluation against 35 quality criteria
- Plan structure assessment (granularity, completeness, TDD)
- Structured feedback (BLOCKING vs SUGGESTIONS)
- Verification that plans are ready for execution

**References:**
- Agent: `${CLAUDE_PLUGIN_ROOT}agents/plan-reviewer.md`
- Skill: `${CLAUDE_PLUGIN_ROOT}skills/conducting-plan-review/SKILL.md`
- Template: `${CLAUDE_PLUGIN_ROOT}templates/plan-evaluation-template.md`
- Integration: Use before `/execute` to validate plans
