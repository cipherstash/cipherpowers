---
name: Getting Started with Skills (CipherPowers)
description: CipherPowers-specific skill discovery using custom find-skills tool
when_to_use: when starting any conversation
version: 1.0.0
extends: ${SUPERPOWERS_SKILLS_ROOT}/skills/using-skills/SKILL.md
---

# Getting Started with Skills (CipherPowers)

This skill extends the upstream superpowers using-skills skill with CipherPowers-specific tool paths and conventions.

**For complete usage instructions, see the upstream skill:**
`${SUPERPOWERS_SKILLS_ROOT}/skills/using-skills/SKILL.md`

The upstream skill covers:
- Critical rules (Read before announcing, follow workflows, use TodoWrite)
- Mandatory workflow before ANY task
- Skills with checklists
- Announcing skill usage
- Instructions ≠ permission to skip workflows

## CipherPowers-Specific Tool Paths

**Tool paths (use these when you need to search for or run skills):**
- find-skills: ./tools/find-skills (searches BOTH cipherpowers and superpowers)
- skill-run: /Users/tobyhede/.config/superpowers/skills/skills/using-skills/skill-run

**Upstream reference paths:**
- upstream find-skills: /Users/tobyhede/.config/superpowers/skills/skills/using-skills/find-skills (superpowers only)
- superpowers skills: /Users/tobyhede/.config/superpowers/skills/skills/

**Discovery workflow for CipherPowers:**
1. Run `./tools/find-skills [PATTERN]` to search both skill repositories
2. Use `--local` flag for cipherpowers-only search
3. Use `--upstream` flag for superpowers-only search
4. Read skills with full path (e.g., `${SUPERPOWERS_SKILLS_ROOT}/skills/category/name/SKILL.md` or local `skills/category/name/SKILL.md`)

## CipherPowers Architecture

When working in this repository, understand the three-layer architecture:

1. **Skills layer** (`skills/`) - Reusable process knowledge (universal workflows)
2. **Automation layer** (`commands/`, `agents/`) - Slash commands and specialized agents
3. **Documentation layer** (`docs/`) - Standards, practices, and reference materials

**Key principle:** Skills are universal, practices are project-specific. Commands dispatch to skills with project context.

See `CLAUDE.md` for complete architecture documentation.
