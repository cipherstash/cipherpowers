# Installing CipherPowers for Codex

Quick setup to enable cipherpowers skills in Codex.

## Installation

1. **Clone cipherpowers repository**:
   ```bash
   mkdir -p ~/.codex/cipherpowers
   cd ~/.codex/cipherpowers
   git clone https://github.com/cipherstash/cipherpowers.git .
   ```

2. **Create personal skills directory** (optional):
   ```bash
   mkdir -p ~/.codex/skills
   ```

3. **Update ~/.codex/AGENTS.md** to include this cipherpowers section:
   ```markdown
   ## CipherPowers System

   <EXTREMELY_IMPORTANT>
   You have cipherpowers. CipherPowers teach you new skills and capabilities. RIGHT NOW run: `~/.codex/cipherpowers/.codex/cipherpowers-codex bootstrap` and follow the instructions it returns.
   </EXTREMELY_IMPORTANT>
   ```

## Verification

Test the installation:
```bash
~/.codex/cipherpowers/.codex/cipherpowers-codex bootstrap
```

You should see skill listings and bootstrap instructions. The system is now ready for use.

## Usage

**List available skills:**
```bash
~/.codex/cipherpowers/.codex/cipherpowers-codex find-skills
```

**Load a specific skill:**
```bash
~/.codex/cipherpowers/.codex/cipherpowers-codex use-skill brainstorming
```

**Run full bootstrap:**
```bash
~/.codex/cipherpowers/.codex/cipherpowers-codex bootstrap
```

## Creating Personal Skills

Personal skills in `~/.codex/skills/` override cipherpowers skills with the same name.

Structure:
```
~/.codex/skills/
└── my-skill/
    └── SKILL.md
```

SKILL.md format:
```markdown
---
name: My Custom Skill
description: Brief description
when_to_use: When to apply this skill
---

# My Custom Skill

[Skill content here]
```

## Integration with CipherPowers Standards

While Codex doesn't have access to the full cipherpowers plugin automation (agents, commands), you can still reference the practices and standards:

**Standards location:** `~/.codex/cipherpowers/plugin/standards/`

Common standards:
- `code-review.md` - Code review severity levels and process
- `conventional-commits.md` - Commit message format
- `git-guidelines.md` - Git workflow standards
- `workflow.md` - Workflow execution patterns

You can read these files directly when needed for context on project conventions.
