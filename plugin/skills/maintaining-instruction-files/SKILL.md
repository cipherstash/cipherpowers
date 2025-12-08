---
name: maintaining-instruction-files
description: Use when creating, updating, or validating CLAUDE.md or AGENTS.md files - ensures size limits, progressive disclosure via docs/ references, multi-agent compatibility, and tool-first content
version: 1.0.0
---

# Maintaining Instruction Files

## Overview

Specialized workflow for AI instruction files (CLAUDE.md, AGENTS.md). Different from general documentation:
- **Size-constrained**: <200 lines ideal, <300 max
- **AI-audience**: Directive style, third-person
- **Progressive disclosure**: Reference docs/, don't include content
- **Multi-agent**: AGENTS.md standard works with any AI assistant

**Why this matters:** Quality instruction files create a virtuous cycle - better context leads to better AI output, which builds trust, which leads to more investment in refined guidance. Poorly maintained instruction files cause the opposite: AI makes mistakes, trust erodes, teams abandon the approach.

**Announce at start:** "I'm using the maintaining-instruction-files skill to work on this instruction file."

**Critical Infrastructure Warning:** Instruction files are NOT documentation - they're critical infrastructure that shapes every AI interaction. A single poorly-thought-out line can mislead the AI in every session. Never accept auto-generated instruction files without careful manual review and curation.

## When to Use

**Use this skill when:**
- Creating new CLAUDE.md or AGENTS.md
- Instruction file exceeds 200 lines (warning) or 300 lines (action required)
- Adding content to instruction files
- Migrating CLAUDE.md to AGENTS.md standard
- Auditing instruction file quality
- Reviewing PRs that modify instruction files

**When NOT to use:**
- General documentation updates (use maintaining-docs-after-changes)
- Creating docs/ content (use organizing-documentation)
- Trivial typo fixes

## Quick Reference

| Size | Status | Action |
|------|--------|--------|
| <200 lines | ✅ Good | Maintain quality |
| 200-300 lines | ⚠️ Warning | Consider extraction |
| >300 lines | ❌ Action required | Run extraction workflow |

| Principle | Implementation |
|-----------|----------------|
| Size discipline | Count lines, extract if >300 |
| Universal relevance | Remove edge cases |
| Tool-first | Reference skills/linters, not rules |
| Reference, don't include | Link to docs/, don't duplicate |
| Multi-agent neutral | No "Claude should...", use AGENTS.md |

## Core Principles

### 0. Context Window is Precious

The AI's context window is limited and valuable - fill it with high-value information, not exhaustive minutiae. Every token spent on instruction files is a token not available for actual work context.

**Research finding:** As instruction count increases, model performance in following them degrades linearly. Even frontier models show this effect. Smaller models are especially prone to instruction overload.

**Claude Code behavior:** Claude's system actively tells the model to skip context files unless they're highly relevant to the current task. If most content appears irrelevant, the model may ignore the entire file. This means bloated instruction files can be worse than no instruction file at all.

### 1. Size Discipline

```bash
# Check current size
wc -l AGENTS.md CLAUDE.md 2>/dev/null || wc -l CLAUDE.md
```

**Thresholds:**
- <200 lines: Ideal, focused and effective
- 200-300 lines: Warning, review for extraction opportunities
- >300 lines: Must extract content to docs/

**Optimization target:** Minimize instruction count, not just line count. Fewer well-chosen instructions outperform many rules. Aim for the minimum guidance that covers essential context.

### 2. Universal Relevance

Every line must apply to **most tasks**. Test each section:
- "Does this apply when fixing bugs?" → Must be Yes
- "Does this apply when adding features?" → Must be Yes
- "Is this only relevant for [specific scenario]?" → If Yes, extract to docs/

### 3. Tool-First Content

**Wrong:** Listing style rules in instruction file
```markdown
## Code Style
- Use camelCase for variables
- 2-space indentation
- No trailing whitespace
```

**Right:** Reference tools
```markdown
## Code Style

Use project linters and formatters. Run `npm run lint` before commits.
```

### 4. Reference, Don't Include (Progressive Disclosure)

This principle implements **progressive disclosure** - a pattern where detailed information is available but not loaded until needed.

**Why it works:**
- AI assistants can follow links to fetch detailed docs when relevant
- Main instruction file stays light, focused on universal context
- Detailed instructions are available but not consuming context window space in every session
- AI decides what to fetch based on current task

**Pattern:**
1. Keep 2-3 sentence summary in instruction file
2. Link to detailed doc with "See [path] for details"
3. AI fetches detailed doc only when working on related tasks

**Wrong:** Duplicating content
```markdown
## Architecture
[50 lines of architecture details]
```

**Right:** Linking to docs
```markdown
## Architecture

High-level: [2-3 sentences]

See `docs/UNDERSTAND/architecture.md` for detailed architecture.
```

### 5. On-Demand Knowledge via Platform Tools

Extend the tool-first principle beyond linters to full platform capabilities:

**Available platform tools:**
- **Skills:** Reusable workflows invoked when needed (`Skill(skill: "name")`)
- **Hooks:** Automated actions triggered by events (pre-commit, session-start)
- **MCP servers:** External tools and data sources
- **Slash commands:** User-triggered context injection

**Pattern:** Instead of front-loading all possible context, configure tools to provide context on-demand:

**Wrong:** Putting everything in instruction file
```markdown
## API Guidelines
[100 lines of API patterns, error handling, authentication...]
```

**Right:** Reference skill that provides guidance when needed
```markdown
## API Guidelines

Use project API patterns. Skill `cipherpowers:api-patterns` provides detailed guidance.
```

**Benefits:**
- Instruction file stays concise
- Detailed guidance available when actually needed
- Context window used efficiently
- Guidance can be updated in one place (the skill/tool)

### 6. Multi-Agent Neutral

**Wrong:** Claude-specific
```markdown
Claude should always check tests before...
```

**Right:** Agent-neutral
```markdown
Always check tests before...
```

## Validation Checklist

**Run before completing any instruction file work:**

- [ ] Line count <300 (warn >200): `wc -l [file]`
- [ ] No edge-case instructions (extract to docs/)
- [ ] Commands actually work (execute to verify)
- [ ] References use correct paths (`${CLAUDE_PLUGIN_ROOT}` or relative)
- [ ] No duplicated content from standards/ or docs/
- [ ] Links to docs/BUILD/00-START/ for prerequisites
- [ ] Links to docs/LOOKUP/ for quick references
- [ ] Agent-neutral wording (no "Claude should...")

## Extraction Workflow

**When instruction file exceeds 300 lines:**

### Step 1: Categorize Content

Review each section and categorize:

| Category | Target Location | Criteria |
|----------|-----------------|----------|
| Prerequisites | docs/BUILD/00-START/ | Required before any work |
| Deep dives | docs/UNDERSTAND/ | Detailed explanations |
| Quick refs | docs/LOOKUP/ | < 30 second lookups |
| Edge cases | docs/BUILD/02-IMPLEMENT/ | Specific scenarios |

### Step 2: Extract Content

Move content to appropriate docs/ location:

```markdown
# Before (in AGENTS.md - 50 lines)
## Database Schema
[Detailed schema documentation...]

# After (in AGENTS.md - 3 lines)
## Database

See `docs/UNDERSTAND/database-schema.md` for schema documentation.
```

### Step 3: Verify References

- All extracted content accessible via links
- No broken references
- Index files updated if applicable

### Step 4: Recount

```bash
wc -l AGENTS.md
```

Should now be <300 (ideally <200).

## Multi-Agent Compatibility

### AGENTS.md Standard

AGENTS.md is the open standard recognized by:
- Claude Code (Anthropic)
- GitHub Copilot
- Cursor
- Sourcegraph Cody
- Other AI assistants

### Relationship Patterns

**Pattern A: Universal file only**
```
AGENTS.md ← All instructions here
CLAUDE.md  (symlink → AGENTS.md)
```

**Pattern B: Universal + extensions**
```
AGENTS.md ← Universal instructions
CLAUDE.md ← Includes @AGENTS.md + Claude-specific
```

**Pattern C: Claude-only (legacy)**
```
CLAUDE.md ← All instructions
```
Recommend migrating to Pattern A or B.

### Migration from CLAUDE.md to AGENTS.md

1. Audit CLAUDE.md for Claude-specific content
2. Extract universal content to new AGENTS.md
3. Either:
   - Create symlink: `ln -s AGENTS.md CLAUDE.md`
   - Or keep CLAUDE.md with `@AGENTS.md` include

### Platform-Specific Tuning

Different AI systems have different quirks. A Claude-optimized instruction file may not work as effectively with GitHub Copilot directly, and vice versa.

**Keep core content the same** across platforms, but be prepared to tweak:
- Context window handling differs
- Prompt format preferences vary
- Tool/skill integration differs

**When to adapt:**
- If AI consistently misinterprets instructions on one platform
- If certain sections seem ignored on specific platforms
- If switching between platforms produces notably different results

**Adaptation strategy:**
- Keep shared AGENTS.md as single source of truth
- Use platform-specific extensions (CLAUDE.md) for platform-specific tweaks
- Test instructions on each platform you support

## Iterative Refinement

**Instruction files are living documents.** Update them as your project evolves and as you learn what helps the AI most.

**Review triggers:**
- After onboarding new team members (they reveal unclear instructions)
- After major feature changes (architecture may need updating)
- When AI makes repeated mistakes (instructions may be unclear or missing)
- Quarterly review (prevent drift and bloat)

**Refinement process:**
1. Identify patterns in AI mistakes or confusion
2. Test instruction changes with real tasks
3. Remove instructions that aren't consistently helping
4. Add instructions only when there's a demonstrated need

**Avoid:** Adding instructions preemptively "just in case" - this leads to bloat. Add based on observed need.

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| "This rule is important" | If not universal, it goes in docs/ |
| "It's only 350 lines" | Models ignore bloated files - attention drops linearly |
| "I'll trim it later" | Size grows, never shrinks without active management |
| "I need it all in one file" | Progressive disclosure via references works better |
| "Users expect comprehensive docs" | Users expect working AI - bloat prevents that |
| "These are all critical rules" | Test: applies to bug fixes AND features? If not, extract |

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Adding rules instead of tool references | Point to linters, skills, hooks |
| Including full content from docs | Link with brief summary only |
| Using Claude-specific language | Rewrite agent-neutral |
| Ignoring line count | Check `wc -l` every edit |
| Creating parallel hierarchy | Reference existing docs/ structure |
| Not verifying commands | Execute each command to test |

## Related Skills

- `${CLAUDE_PLUGIN_ROOT}skills/maintaining-docs-after-changes/SKILL.md` - General doc sync
- `${CLAUDE_PLUGIN_ROOT}skills/capturing-learning/SKILL.md` - Adding lessons learned
- `${CLAUDE_PLUGIN_ROOT}skills/organizing-documentation/SKILL.md` - docs/ structure

## References

- Template: `${CLAUDE_PLUGIN_ROOT}templates/agents-md-template.md`
- Documentation Structure: `${CLAUDE_PLUGIN_ROOT}standards/documentation-structure.md`
- Best Practices Research: `agents-md-best-practices.md`
