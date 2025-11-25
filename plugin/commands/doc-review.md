# Documentation Review

Review and update project documentation to ensure it stays synchronized with recent code changes.

<instructions>
## Instructions

## MANDATORY: Skill Activation

**Load skill context:**
@${CLAUDE_PLUGIN_ROOT}skills/maintaining-docs-after-changes/SKILL.md

**Step 1 - EVALUATE:** State YES/NO for skill activation:
- Skill: "cipherpowers:maintaining-docs-after-changes"
- Applies to this task: YES/NO (reason)

**Step 2 - ACTIVATE:** If YES, use Skill tool NOW:
```
Skill(skill: "cipherpowers:maintaining-docs-after-changes")
```

⚠️ Do NOT proceed without completing skill evaluation and activation.

---

1. **Use the technical-writer agent:**
   - Agent: `technical-writer`
   - This agent follows the maintaining-docs-after-changes skill:
     - Phase 1: Analysis (review changes, check docs, identify gaps)
     - Phase 2: Update (modify content, restructure, verify completeness)

2. **The agent references project documentation standards:**
   - `${CLAUDE_PLUGIN_ROOT}standards/documentation.md` - Formatting and completeness standards

**Key Principle:**
Maintaining existing documentation after code changes is NOT "proactively creating docs" - it's keeping current docs accurate. If code changed, docs MUST update.

**Why this structure?**
- Agent = Enforces workflow with non-negotiable steps
- Skill = Universal process (part of cipherpowers)
- Practices = Project-specific standards (your docs format)
- Command = Thin dispatcher (adds project context)
</instructions>
