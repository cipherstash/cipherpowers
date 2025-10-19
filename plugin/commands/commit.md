# Commit

Use the commit-workflow skill for systematic commit process.

<instructions>
## Instructions

1. **Follow the commit-workflow skill:**
   - Read: `${CLAUDE_PLUGIN_ROOT}commit-workflow/SKILL.md`
   - This skill contains the complete workflow:
     - Pre-commit checks (linters, formatters, tests)
     - Staging and diff review
     - Atomic commit composition
     - Conventional commit message formatting

2. **The skill references project practices:**
   - `${CLAUDE_PLUGIN_ROOT}practices/conventional-commits.md` - Message format
   - `${CLAUDE_PLUGIN_ROOT}practices/git-guidelines.md` - Git workflow standards

**Why this structure?**
- Skill = Discoverable workflow (automatically available via Skill tool)
- Practices = Project-specific standards (conventional commit rules)
- Command = Thin dispatcher (user entry point)
</instructions>
