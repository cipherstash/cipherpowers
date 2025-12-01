# Revise

Implement findings from verification reports. Works with collation reports produced by `/verify`.

**Core principle:** Separate what to fix (verify) from how to fix it (revise).

## Usage

```
/cipherpowers:revise [scope] [collation-file]
```

**Scope options:**
- `common` - Implement common issues only (immediate)
- `exclusive` - Implement VALIDATED exclusive issues (after cross-check)
- `all` - Implement all actionable (default)

## Algorithmic Workflow

**Decision tree (follow exactly, no interpretation):**

1. Is this a revise request?
   - YES → Continue to step 2
   - NO → This command was invoked incorrectly

2. **MANDATORY: Skill Activation**

**Load skill context:**
@${CLAUDE_PLUGIN_ROOT}skills/revising-findings/SKILL.md

**Step 1 - EVALUATE:** State YES/NO for skill activation:
- Skill: "cipherpowers:revising-findings"
- Applies to this task: YES/NO (reason)

**Step 2 - ACTIVATE:** If YES, use Skill tool NOW:
```
Skill(skill: "cipherpowers:revising-findings")
```

⚠️ Do NOT proceed without completing skill evaluation and activation.

3. **FOLLOW THE SKILL EXACTLY:**
   - Locate collation report
   - Check cross-check status (for exclusive/all)
   - Build implementation list
   - Handle UNCERTAIN issues
   - Dispatch implementation agents
   - Verify implementation

4. **STOP when revise is complete.**

## Related Commands

- `/cipherpowers:verify` - Generate collation reports (prerequisite)

## Related Skills

- `revising-findings` - Core implementation workflow
- `dual-verification` - Verification pattern producing collation reports

## Remember

- `/revise common` works immediately after collation
- `/revise exclusive` requires cross-check complete
- Skill contains detailed workflow - follow it exactly
