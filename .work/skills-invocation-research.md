# Skills Invocation Research

> Last Updated: 2025-11-25

Research findings on Claude Code skill invocation reliability and best practices.

## The Skill Tool

Claude Code has a Skill tool with this signature:

```
name: "Skill"
parameters:
  skill: "The skill name (no arguments). E.g., \"pdf\" or \"xlsx\""
```

The colon-separated format (`plugin-name:skill-name`) is the **fully qualified name** for plugin skills. Example: `cipherpowers:dual-verification-review`

## The Problem: Skills Don't Activate Reliably

Research by Scott Spence found:

- **Simple instructions yield ~20% activation rate** - essentially a coin flip
- **Model autonomy is unreliable** - Claude is supposed to autonomously decide when to use skills based on context, but this is inconsistent
- **Prose instructions don't force invocation** - Writing "Use Skill tool with: skill: '...'" is just text; it doesn't guarantee Claude will actually call the tool

The official documentation states skills are "model-invoked" - Claude autonomously decides when to use them. In practice, this autonomous activation is unreliable.

## Solutions That Work

### 1. Forced Evaluation (84% success rate)

Forces Claude to explicitly evaluate and commit before proceeding:

```markdown
## MANDATORY: Skill Activation

**Step 1 - EVALUATE:** State YES/NO for skill activation:
- Skill: "cipherpowers:skill-name"
- Applies to this task: YES/NO (reason)

**Step 2 - ACTIVATE:** If YES, use Skill tool NOW before proceeding.

⚠️ Do NOT proceed without completing skill evaluation and activation.
```

**Why it works:** Creates commitment through visible evaluation. Claude must show its reasoning before implementation, creating accountability.

### 2. Direct @ Reference (fallback guarantee)

Reference skill content directly using @ syntax:

```markdown
@${CLAUDE_PLUGIN_ROOT}skills/skill-name/SKILL.md
```

This ensures skill content is loaded into context even if the Skill tool isn't invoked.

### 3. Recommended: Use Both

For maximum reliability, combine both approaches:

```markdown
## MANDATORY: Skill Activation

**Load skill context:**
@${CLAUDE_PLUGIN_ROOT}skills/skill-name/SKILL.md

**Step 1 - EVALUATE:** State YES/NO for skill activation:
- Skill: "cipherpowers:skill-name"
- Applies to this task: YES/NO (reason)

**Step 2 - ACTIVATE:** If YES, use Skill tool NOW:
```
Skill(skill: "cipherpowers:skill-name")
```

⚠️ Do NOT proceed without completing skill evaluation and activation.
```

This provides:
- **@ reference**: Guarantees skill content is in context
- **Forced evaluation**: 84% activation rate through commitment
- **Explicit syntax**: Clear instruction on how to activate

## Key Insights

1. **Aggressive language helps** - Words like "MANDATORY", "CRITICAL", "NOW" improve compliance
2. **Structured evaluation beats passive suggestion** - Requiring explicit YES/NO statements creates accountability
3. **Eliminate passive suggestions** - Don't say "use skills if they match"; require explicit evaluation
4. **Redundancy is good** - Both @ reference AND Skill tool invocation ensures content is available

## Sources

- [Agent Skills - Claude Code Docs](https://code.claude.com/docs/en/skills)
- [Inside Claude Code Skills: Structure, prompts, invocation - Mikhail Shilkov](https://mikhail.io/2025/10/claude-code-skills/)
- [How to Make Claude Code Skills Activate Reliably - Scott Spence](https://scottspence.com/posts/how-to-make-claude-code-skills-activate-reliably)
- [Claude Agent Skills: A First Principles Deep Dive](https://leehanchung.github.io/blogs/2025/10/26/claude-skills-deep-dive/)
