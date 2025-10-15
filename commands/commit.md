# Commit

Use the commit-workflow skill for systematic commit process.

<instructions>
## Instructions

1. **Follow the commit-workflow skill:**
   - Read: `@skills/commit-workflow/SKILL.md`
   - This skill contains the complete workflow:
     - Pre-commit checks (linters, formatters, tests)
     - Staging and diff review
     - Atomic commit composition
     - Conventional commit message formatting

2. **The skill references project practices:**
   - `@docs/practices/conventional-commits.md` - Message format
   - `@docs/practices/git-guidelines.md` - Git workflow standards

**Why this structure?**
- Skill = Discoverable workflow (agents can find it with find-skills)
- Practices = Project-specific standards (conventional commit rules)
- Command = Thin dispatcher (user entry point)
</instructions>
