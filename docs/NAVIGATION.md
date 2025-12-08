# CipherPowers Navigation

Task-based navigation for CipherPowers documentation.

## I want to...

### Start a New Feature

1. **Brainstorm** the design
   - Command: `/cipherpowers:brainstorm`
   - Skill: `cipherpowers:brainstorming`
   - Guide: [WORKFLOW.md](../plugin/docs/WORKFLOW.md#1-brainstorm)

2. **Plan** the implementation
   - Command: `/cipherpowers:plan`
   - Skill: `cipherpowers:writing-plans`
   - Guide: [WORKFLOW.md](../plugin/docs/WORKFLOW.md#2-plan)

3. **Execute** the plan
   - Command: `/cipherpowers:execute [plan-file]`
   - Skill: `cipherpowers:executing-plans`
   - Guide: [WORKFLOW.md](../plugin/docs/WORKFLOW.md#3-execute)

### Review Code

- **Request review:** `/cipherpowers:code-review`
- **Skill:** `cipherpowers:conducting-code-review`
- **Reference:** [COMMANDS.md](../plugin/docs/COMMANDS.md)

### Commit Changes

- **Command:** `/cipherpowers:commit`
- **Skill:** `cipherpowers:commit-workflow`
- **Reference:** [COMMANDS.md](../plugin/docs/COMMANDS.md)

### Verify Work

| What | Command | Use When |
|------|---------|----------|
| Plan quality | `/cipherpowers:verify plan` | Before executing a plan |
| Code quality | `/cipherpowers:verify code` | Before merging |
| Plan adherence | `/cipherpowers:verify execute` | After executing a plan |
| Documentation | `/cipherpowers:verify docs` | Before merging |

### Find a Skill

- **Browse by category:** [SKILLS.md](../plugin/docs/SKILLS.md)
- **Invoke directly:** `Skill(skill: "cipherpowers:skill-name")`

### Understand the Architecture

- **Overview:** [CLAUDE.md](../CLAUDE.md)
- **Three layers:** Skills → Commands/Agents → Standards
- **Plugin structure:** [CLAUDE.md](../CLAUDE.md#directory-structure)

### Contribute to CipherPowers

1. Read [CLAUDE.md](../CLAUDE.md) for architecture
2. Use templates from `plugin/templates/`
3. Follow TDD for new skills
4. Use `/cipherpowers:commit` for commits

## Quick Reference

| Task | Command |
|------|---------|
| Design refinement | `/cipherpowers:brainstorm` |
| Create plan | `/cipherpowers:plan` |
| Execute plan | `/cipherpowers:execute [file]` |
| Code review | `/cipherpowers:code-review` |
| Commit | `/cipherpowers:commit` |
| Verify | `/cipherpowers:verify [type]` |
| Capture learning | `/cipherpowers:summarise` |

## See Also

- [INDEX.md](INDEX.md) - Documentation index with purpose column
- [plugin/docs/README.md](../plugin/docs/README.md) - Plugin documentation hub
