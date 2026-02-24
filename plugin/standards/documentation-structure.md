---
name: Documentation Structure
description: Organize project documentation by developer intent (build, fix, understand, lookup) rather than content type for faster navigation and reduced cognitive load.
when_to_use: when setting up project documentation, reorganizing existing docs, or onboarding new team members
applies_to: all projects with non-trivial documentation needs
version: 1.0.0
---

# Documentation Structure

Organize documentation by **developer intent** rather than content type. Developers approach docs with a task in mind - structure should match that mental model.

## Core Principle

**Bad:** Organize by content type (architecture/, testing/, api/)
**Good:** Organize by what developer wants to do (BUILD/, FIX/, LOOKUP/)

## Directory Structure

```
docs/
├── BUILD/           # "I need to implement something"
│   ├── 00-START/    # Prerequisites before ANY work
│   ├── 01-DESIGN/   # Research, decisions, domain knowledge
│   ├── 02-IMPLEMENT/# Patterns and how-to guides
│   ├── 03-TEST/     # Testing strategies
│   └── 04-VERIFY/   # Checklists and validation
│
├── FIX/             # "Something is broken"
│   ├── symptoms/    # Organized by what you SEE
│   ├── investigation/ # Systematic debugging approaches
│   └── solutions/   # Known fixes catalog
│
├── UNDERSTAND/      # "I need to learn how this works"
│   ├── core-systems/# Architecture deep dives
│   ├── evolution/   # Why things are the way they are
│   └── rendering-systems/ # Domain-specific understanding
│
├── LOOKUP/          # "I need a quick reference" (< 30 seconds)
│   ├── commands.md
│   ├── shortcuts.md
│   └── error-codes.md
│
└── INDEX.md         # Master index with purpose annotations
```

## Directory Purposes

### BUILD/ - Implementation Workflow

Mirrors natural development phases. Numbered prefixes enforce reading order.

| Directory | Purpose | Example Content |
|-----------|---------|-----------------|
| `00-START/` | **Mandatory** prerequisites | Prime directives, coordinate systems, architecture overview |
| `01-DESIGN/` | Research and decisions | Domain knowledge, solution selection, design docs |
| `02-IMPLEMENT/` | How to build things | Patterns, code examples, best practices |
| `03-TEST/` | How to verify things | Testing strategies, test patterns |
| `04-VERIFY/` | Final checks | Checklists, validation procedures |

### FIX/ - Symptom-Based Debugging

Developers search by what they **see**, not by root cause.

```
FIX/
├── symptoms/
│   ├── visual-bugs/       # "Rendering looks wrong"
│   ├── physics-bugs/      # "Movement is broken"
│   ├── test-failures/     # "Tests fail unexpectedly"
│   └── performance/       # "It's slow"
├── investigation/
│   └── systematic-debugging.md
└── solutions/
    └── common-errors-reference.md
```

**Template:** `${CLAUDE_PLUGIN_ROOT}templates/symptom-debugging-template.md`
**Skill:** `${CLAUDE_PLUGIN_ROOT}skills/documenting-debugging-workflows/SKILL.md`

### UNDERSTAND/ - Deep Learning

For when you need context, not just instructions.

- **core-systems/** - How the architecture works
- **evolution/** - Why decisions were made (historical context)
- **[domain]-systems/** - Domain-specific deep dives

### LOOKUP/ - Quick Reference

**Rule:** If it takes > 30 seconds to find, it's not a lookup.

- Keyboard shortcuts
- Command references
- ID registries
- Error code tables
- Quick reference cards (one-pagers)

## Index File Pattern

Every docs root should have `INDEX.md` with:

```markdown
# Project Documentation Index

**Last Updated:** YYYY-MM-DD
**Total Documents:** N

## BUILD
| File | Title | Purpose |
|------|-------|---------|
| `BUILD/00-START/architecture.md` | Architecture Overview | Entry point for understanding system |

## FIX
...

## UNDERSTAND
...

## LOOKUP
...

## Excluded (Not Indexed)
- `archive/` - Historical content
- `work/` - Work logs and drafts
- `plans/` - Implementation plans
```

## What NOT to Include

Keep these **outside** the main structure:

| Directory | Content | Why Excluded |
|-----------|---------|--------------|
| `archive/` | Obsolete docs | Clutters search |
| `work/` | Daily logs, drafts | Transient |
| `plans/` | Implementation plans | Task-specific |
| `code-review/` | Review feedback | Session-specific |

## Migration Strategy

When reorganizing existing docs:

1. **Audit** - List all existing docs with their actual purpose
2. **Map** - Assign each to BUILD/FIX/UNDERSTAND/LOOKUP
3. **Redirect** - Create README.md in old locations pointing to new
4. **Index** - Build INDEX.md as you migrate
5. **Verify** - Check all internal links still work

## README-Per-Directory Pattern

Every directory MUST have a `README.md` with consistent structure:

```markdown
# [CATEGORY]: [Brief Description]

**Purpose:** [1-2 sentence description]

**Use this section when:**
- [Use case 1]
- [Use case 2]

---

## Navigation

[Content organized by intent]

---

## Quick Navigation

- **Ready to build?** → [Link to next stage]
- **Need to understand?** → [Link to UNDERSTAND]

---

## Files in This Directory

- **[file.md]** - [Description]
```

READMEs serve as both index and gateway. They enable self-service navigation.

**Template:** `${CLAUDE_PLUGIN_ROOT}templates/documentation-readme-template.md`

## Dual Navigation Pattern

Maintain two parallel navigation systems:

| Navigation | Purpose | Usage |
|------------|---------|-------|
| `NAVIGATION.md` | Task-based primary (80%) | "What do I want to do?" |
| `INDEX.md` | Concept-based fallback (20%) | "I know the term" |

Both must be maintained in parallel. INDEX.md should include an **Excluded Categories** list (Code Reviews, Plans, Work Logs, Archives) to prevent clutter.

## Document Type Naming Conventions

File naming signals purpose before opening:

| Pattern | Purpose | Example |
|---------|---------|---------|
| `ALLCAPS.md` | Document type | `SUMMARY.md`, `QUICK-REFERENCE.md`, `README.md` |
| `00-NAME/` | Sequence enforcement | `00-START/`, `01-DESIGN/`, `02-IMPLEMENT/` |
| `lowercase-dashes.md` | Content docs | `thrust-mechanics.md`, `bevy-ecs-patterns.md` |
| `*-patterns.md` | Pattern collection | `ecs-patterns.md` |
| `*-checklist.md` | Verification list | `pre-merge-checklist.md` |

ALLCAPS filenames stand out in directory listings and are predictable across projects.

## Research Package Pattern

Bundle complex domain knowledge as self-contained modules:

```
propulsion/
├── 00-START-HERE.md           # Entry point + verification status
├── README.md                   # Package overview + TL;DR
├── how-to-use-this.md         # Navigation guide with reading paths
├── VERIFICATION-REVIEW.md      # Accuracy audit
├── [topic-documents].md        # Research content
└── QUICK-REFERENCE.md          # One-page summary
```

**Key features:**
- Multiple entry points (different time budgets)
- TL;DR for time-constrained readers
- Verification status tracked
- Self-contained (can be shared independently)

**Template:** `${CLAUDE_PLUGIN_ROOT}templates/research-package-template.md`
**Skill:** `${CLAUDE_PLUGIN_ROOT}skills/creating-research-packages/SKILL.md`

## Cross-Referencing Strategy

Use multiple link types for discoverability:

```markdown
# Inline Cross-Reference
**See also:** [ECS Architecture](path) for comprehensive guide.

# Scenario-Based Links
- **Stuck on a bug?** → See [FIX](path) for debugging strategies
- **Need to understand a system?** → See [UNDERSTAND](path)

# Workflow Navigation
- **Ready to design?** → [01-DESIGN](path)
- **Ready to implement?** → [02-IMPLEMENT](path)
```

Links should include context describing WHY to click them.

## Metadata Visibility

Show status, dates, and version at document start:

**Visual indicators:**
| Icon | Meaning |
|------|---------|
| ✅ | Verified/Complete |
| ⚠️ | Needs attention/Draft |
| ☠️ | Dangerous/Fatal |
| 🎯 | Ready for action |

**Required metadata:**
- Status indicator
- Last verified date
- Version (if applicable, e.g., `Bevy Version: 0.17.x`)
- Read time estimate

## Legacy Archive Pattern

Preserve knowledge during major refactorings:

```
legacy-code-extractions/
├── README.md                    # Archive overview + when to reference
├── [feature-name]/
│   ├── COMPLETE-EXTRACTION.md   # Full code + context
│   ├── SUMMARY.md               # Executive summary
│   └── QUICK-REFERENCE.md       # Key formulas + constants
```

Mark as read-only historical reference. Document when to reference (for context) and when NOT to (for implementation).

## Progressive Disclosure

Respect varying time budgets with graduated entry points:

| Time Budget | Entry Point | Content |
|-------------|-------------|---------|
| 5 min | TL;DR section | "Just tell me what's wrong" |
| 20 min | README + key sections | "I need to understand what changed" |
| 1-2 hours | Full package | "I need to fully understand" |

Provide time estimates for each reading path.

## Role-Based Reading Paths

Tailor reading sequences for different reader roles:

```markdown
## If You're a Game Designer
**Goal:** Understand gameplay implications
**Reading order:** README.md → design-decisions.md → gameplay-impact.md
**Time:** ~60 minutes
**Takeaway:** [key insight for this role]

## If You're a Backend Engineer
**Goal:** Understand implementation requirements
**Reading order:** README.md → architecture.md → api-contracts.md
**Time:** ~45 minutes
**Takeaway:** [key insight for this role]
```

## Table-Based Quick Reference

Use scannable tables for rapid lookup:

```markdown
| Symptom | Likely Cause | Investigation |
|---------|--------------|---------------|
| Rendering artifacts | Floating point precision | [Floating Origin](path) |
| Slow startup | Large asset loading | [Asset Pipeline](path) |
```

Include visual icons (✅, ⚠️, ☠️) for quick assessment.

**Template:** `${CLAUDE_PLUGIN_ROOT}templates/quick-reference-template.md`

## Checklist Quality Gates

Structure quality gates with clear pass/fail criteria:

```markdown
### Automated Checks (Must Pass)
- [ ] Unit tests: `cargo test --lib`
- [ ] Integration tests: `cargo nextest run`

### Manual Verification (Should Review)
- [ ] All public APIs have documentation
- [ ] No magic numbers without comments

### If Automated Tests Fail
**STOP. Do not proceed.**
1. Check test output for specific failures
2. Fix issues before continuing
```

Separate "Must Pass" from "Should Review" - make the distinction explicit.

**Template:** `${CLAUDE_PLUGIN_ROOT}templates/verification-checklist-template.md`
**Skill:** `${CLAUDE_PLUGIN_ROOT}skills/creating-quality-gates/SKILL.md`

## Related Skills

For implementing these patterns:

- **Organizing documentation:** `${CLAUDE_PLUGIN_ROOT}skills/organizing-documentation/SKILL.md`
- **Creating research packages:** `${CLAUDE_PLUGIN_ROOT}skills/creating-research-packages/SKILL.md`
- **Documenting debugging workflows:** `${CLAUDE_PLUGIN_ROOT}skills/documenting-debugging-workflows/SKILL.md`
- **Creating quality gates:** `${CLAUDE_PLUGIN_ROOT}skills/creating-quality-gates/SKILL.md`

## Benefits

- **Faster navigation** - Structure matches developer mental model
- **Reduced cognitive load** - Know where to look before searching
- **Better discoverability** - Related docs grouped by intent
- **Clearer ownership** - Each directory has clear scope
- **Easier onboarding** - New developers follow BUILD/00-START path
