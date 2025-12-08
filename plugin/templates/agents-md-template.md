---
name: AGENTS.md Template
description: Template for creating AGENTS.md files that follow multi-agent compatibility best practices
when_to_use: when creating new AGENTS.md for a project, or restructuring an existing oversized instruction file
version: 1.0.0
---

# AGENTS.md Template

## Overview

Template for creating AGENTS.md files - the open standard for AI coding assistant project instructions. This format works with Claude Code, GitHub Copilot, Cursor, and other AI assistants.

## Size Guidelines

| Threshold | Action |
|-----------|--------|
| <200 lines | ✅ Ideal - concise and focused |
| 200-300 lines | ⚠️ Warning - consider extraction |
| >300 lines | ❌ Too large - must extract content |

**Principle:** Every line must be universally applicable to most tasks. Edge cases go in docs/.

## Template Structure

```markdown
<!--
  Note: This project uses the open AGENTS.md standard for AI coding assistant instructions.
  This file works with Claude Code, GitHub Copilot, Cursor, Cody, and other AI assistants.
  A symlinked or extended CLAUDE.md may exist for Claude-specific features.
  See: https://github.com/anthropics/agents-md-spec (or relevant spec link)
-->

# Project Name

Brief description of what this project does (2-3 sentences).

## Quick Start

### Development Commands

- **Test:** `[test command]`
- **Check:** `[lint/typecheck command]`
- **Build:** `[build command]`
- **Run:** `[run command]`

### Setup

[Essential setup steps only - link to detailed docs if needed]

## Architecture

[High-level overview - 5-10 lines max]

See [docs/UNDERSTAND/architecture.md] for detailed architecture documentation.

## Critical Guidelines

[Only universally applicable rules - 10-20 lines max]

- Rule 1
- Rule 2
- Rule 3

**For style/formatting:** Use linters and formatters, not instructions here.

## Key Files

| Path | Purpose |
|------|---------|
| `src/` | Main source code |
| `tests/` | Test files |
| `docs/` | Documentation |

## Security & Special Considerations (Optional)

<!-- Include this section if your project has critical requirements -->

[If applicable, mention anything the AI should be cautious about:]

- **Secrets:** [How secrets are managed, what NOT to log/commit]
- **Compliance:** [Regulatory requirements, data handling rules]
- **Performance:** [Performance-critical sections to be careful with]
- **Security:** [Security-sensitive areas, authentication handling]

Example:
- Do not log or expose API keys - use environment variables
- Payment processing in `src/payments/` requires PCI compliance awareness
- Rate limiting logic in `src/api/` is performance-critical

## See Also

- `docs/BUILD/00-START/` - Prerequisites before any work
- `docs/LOOKUP/` - Quick references
- `CLAUDE.md` - Claude-specific extensions (if applicable)
```

## Section Guidance

### Project Overview (5-10 lines)
- What the project does
- Tech stack summary
- Entry point for understanding

### Quick Start Commands (10-20 lines)
- Essential commands only
- Build, test, run, lint
- Reference docs/ for detailed setup

### Critical Guidelines (20-40 lines)
- Only universally relevant rules
- Point to skills/tools for enforcement
- No edge cases (extract to docs/)

### Architecture Pointers (10-20 lines)
- Links to docs/UNDERSTAND/ for deep dives
- Links to docs/BUILD/00-START/ for prerequisites
- Reference pattern: "See [file] for details"

### See Also (5-10 lines)
- docs/BUILD/00-START/ - Prerequisites
- docs/LOOKUP/ - Quick references
- CLAUDE.md - Claude-specific extensions

## Anti-Patterns

**Don't:**
- Include content from referenced files (link instead)
- List style rules (use linters)
- Include edge-case instructions (extract to docs/)
- Duplicate content from standards/
- Address specific AI assistants ("Claude should...")
- Include lengthy code examples (link to examples/)

**Do:**
- Keep content universally applicable
- Reference detailed docs via links
- Use neutral wording (works with any AI)
- Point to tools for enforcement
- Keep under 200 lines

## Multi-Agent Compatibility

This template follows the AGENTS.md open standard:
- Works with Claude Code, GitHub Copilot, Cursor, Cody, and others
- Agent-neutral wording (no "Claude should...")
- References skills/tools instead of hardcoding rules

**Relationship with CLAUDE.md:**
- AGENTS.md = Universal instructions (any AI)
- CLAUDE.md = Claude-specific extensions (can include `@AGENTS.md`)
- Or: Single AGENTS.md with symlink from CLAUDE.md

## Related

- Skill: `cipherpowers:maintaining-instruction-files` - Workflow for maintaining these files
- Standard: `${CLAUDE_PLUGIN_ROOT}standards/documentation-structure.md` - docs/ organization
