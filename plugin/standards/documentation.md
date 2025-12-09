---
name: Documentation Standards
description: Rules for documentation formatting, structure, and organization
when_to_use: when creating or updating documentation
applies_to: all projects
related_skills:
  - organizing-documentation
  - maintaining-docs-after-changes
  - maintaining-instruction-files
  - capturing-learning
version: 2.0.0
---

# Documentation standards

## Structure

| Directory | Purpose |
|-----------|---------|
| `BUILD/` | How to create |
| `FIX/` | How to resolve (by symptom) |
| `UNDERSTAND/` | How it works |
| `LOOKUP/` | Quick reference (<30s) |

## Formatting

- Sentence case for headings
- Maximum nesting: 3 levels
- Status indicators: ✅ good, ⚠️ caution, ❌ avoid

## Instruction files

| Size | Status |
|------|--------|
| <200 lines | ✅ |
| 200-300 | ⚠️ |
| >300 | ❌ |

## Naming

| Pattern | Example |
|---------|---------|
| ALLCAPS | `SUMMARY.md` |
| Numeric prefix | `00-START/` |
| lowercase-dashes | `api-patterns.md` |
| README_* | `README_ARCHITECTURE.md` |

## Navigation

- Every docs/ directory: INDEX.md with purpose column
- Cross-reference with relative links
- Moved content: leave redirect

## Anti-patterns

| Don't | Do |
|-------|-----|
| Nest >3 levels | Flatten or split |
| Duplicate content | Reference |
| Tutorials in LOOKUP | Move to BUILD/ |
| FIX by root cause | Organize by symptom |
| Skip INDEX.md | Always create |
| "Document later" | Document with code |

## README checklist

- What and why
- Getting started
- Dependencies
- Commands/tasks
- Examples
- Troubleshooting
- Project structure
- License

## Related skills

| Skill | Use case |
|-------|----------|
| `organizing-documentation` | Set up docs structure |
| `maintaining-docs-after-changes` | Sync after code changes |
| `maintaining-instruction-files` | CLAUDE.md / AGENTS.md |
| `capturing-learning` | Retrospectives |
