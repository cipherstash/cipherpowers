# Session Start Context

This file provides environment context at the beginning of each Claude Code session.

## Plugin Environment

**CLAUDE_PLUGIN_ROOT:** `${pwd}`

This variable points to the root directory of the CipherPowers plugin installation.

## Path Reference Convention

When referencing plugin files in agents, commands, or skills, always use:

```markdown
@${CLAUDE_PLUGIN_ROOT}skills/skill-name/SKILL.md
@${CLAUDE_PLUGIN_ROOT}standards/standard-name.md
@${CLAUDE_PLUGIN_ROOT}principles/principle-name.md
@${CLAUDE_PLUGIN_ROOT}templates/template-name.md
```

**Do NOT use relative paths without the variable:**
```markdown
@skills/...  ❌ Does not work in subagent contexts
```

## Usage

Copy this file to your project's `.claude/context/` directory to inject plugin environment information at session start:

```bash
mkdir -p .claude/context
cp ${CLAUDE_PLUGIN_ROOT}hooks/examples/context/session-start.md \
   .claude/context/session-start.md
```

**Note:** SessionStart is not currently a supported hook in Claude Code. This file serves as a template for injecting environment context via other hooks (e.g., UserPromptSubmit, SlashCommandStart).

## Alternative: User Prompt Hook

If SessionStart hook becomes available, this context will auto-inject. Until then, consider using UserPromptSubmit hook or command-specific context injection.
