# Commit

Use the commit-workflow skill for systematic commit process.

<instructions>
## Instructions

1. **Follow the commit-workflow skill:**
   - Read: `${CLAUDE_PLUGIN_ROOT}skills/commit-workflow/SKILL.md`
   - This skill contains the complete workflow:
     - Pre-commit checks (linters, formatters, tests)
     - Staging and diff review
     - Atomic commit composition
     - Conventional commit message formatting

2. **Use the code-committer agent:**
   - The agent implements the workflow from the skill
   - Ensures non-negotiable steps with persuasion principles
   - Agent will reference the skill automatically

3. **The skill references project practices:**
   - `${CLAUDE_PLUGIN_ROOT}standards/conventional-commits.md` - Message format
   - `${CLAUDE_PLUGIN_ROOT}standards/git-guidelines.md` - Git workflow standards

**Why this structure?**
- Skill = Discoverable workflow (automatically available via Skill tool)
- Practices = Project-specific standards (conventional commit rules)
- Command = Thin dispatcher (user entry point)
</instructions>
