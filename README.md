# CipherPowers

Comprehensive development toolkit for Claude Code, delivered as a modular plugin system. CipherPowers seamlessly integrates three layers: skills (reusable workflows), automation (commands and agents), and documentation (practices and standards) to deliver team-wide consistency and discoverability through the Claude Code plugin architecture.

## Key Commands

**Plan Execution:**
- `/execute [plan-file]` - Execute implementation plans with automatic agent selection, batch-level code review, and retrospective completion

**Code Quality:**
- `/code-review` - Manual code review trigger
- `/commit` - Commit with conventional format

**Documentation:**
- `/doc-review` - Sync documentation with code changes
- `/summarise` - Capture learning and create retrospectives

**Planning:**
- `/brainstorm` - Refine ideas using Socratic method
- `/write-plan` - Create detailed implementation plans

## Discovery Tools

**Find Skills:**
```bash
./plugin/tools/find-skills "pattern"
./plugin/tools/find-skills --local "pattern"    # cipherpowers only
./plugin/tools/find-skills --upstream "pattern" # superpowers only
```

**Find Practices:**
```bash
./plugin/tools/find-practices "pattern"
./plugin/tools/find-practices --local "pattern"    # cipherpowers only
./plugin/tools/find-practices --upstream "pattern" # marketplace only
```

## Direct References

In agents and commands, use:
- `@${CLAUDE_PLUGIN_ROOT}plugin/practices/practice-name.md` - Direct practice reference
- `@${SUPERPOWERS_SKILLS_ROOT}/skills/category/skill-name/SKILL.md` - Skill reference
- `${CLAUDE_PLUGIN_ROOT}practices/practice-name.md` - Relative to plugin root
- `${CLAUDE_PLUGIN_ROOT}skills/category/skill-name/SKILL.md` - Relative to plugin root

## Documentation

See `CLAUDE.md` for complete architecture and usage guide.
