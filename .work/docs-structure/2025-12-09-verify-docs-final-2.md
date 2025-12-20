# Documentation Standards Verification Report #2

## Metadata
- Date: 2025-12-09
- Reviewer: Agent #2
- Subject: plugin/standards/documentation.md
- Ground truth: Four documentation skills

## Executive Summary
- Total issues found: 9
- Critical: 1, High: 3, Medium: 4, Low: 1

The documentation standards file (`plugin/standards/documentation.md`) is significantly incomplete compared to the comprehensive guidance provided in the four documentation skills. The standards focus almost exclusively on formatting and README structure, while the skills teach far more sophisticated workflows, principles, and practices. The standards file appears to be a legacy document that has not been updated to reflect the matured skill-based approach.

## Issues Found

### [Missing]: Intent-Based Documentation Structure (BUILD/FIX/UNDERSTAND/LOOKUP)
- **Location:** Entire standards file - missing concept
- **Severity:** Critical
- **Current content:** The standards file has no mention of intent-based documentation organization
- **Actual content:** The `organizing-documentation` skill teaches a comprehensive intent-based structure:
  - BUILD/ (00-START, 01-DESIGN, 02-IMPLEMENT, 03-TEST, 04-VERIFY)
  - FIX/ (symptoms, investigation, solutions)
  - UNDERSTAND/ (core-systems, evolution)
  - LOOKUP/ (quick references < 30 seconds)
  - INDEX.md with mandatory purpose column
  - NAVIGATION.md for task-based navigation
- **Impact:** Teams using only the standards file will organize docs by content-type (architecture/, testing/) rather than developer intent, leading to poor discoverability
- **Action:** Add comprehensive section on intent-based documentation structure, referencing the organizing-documentation skill

### [Missing]: Two-Phase Documentation Maintenance Workflow
- **Location:** Entire standards file - missing concept
- **Severity:** High
- **Current content:** Standards mention "documentation completeness" as a bullet point but provide no maintenance workflow
- **Actual content:** The `maintaining-docs-after-changes` skill teaches a two-phase workflow:
  - Phase 1: Analysis (git diff review, existing doc check, gap identification)
  - Phase 2: Update (content sync, restructuring, documentation of changes)
  - Explicit checklist: git diff, CLAUDE.md, README*.md, practices docs, examples, usage examples, troubleshooting, cross-references
  - Common rationalizations table explaining why skipping is wrong
- **Impact:** Without a maintenance workflow, documentation drifts from code reality. Teams will not systematically check all documentation locations
- **Action:** Add section on documentation maintenance workflow or explicitly reference the maintaining-docs-after-changes skill

### [Missing]: Instruction File Management (CLAUDE.md/AGENTS.md)
- **Location:** Entire standards file - missing concept
- **Severity:** High
- **Current content:** No mention of CLAUDE.md, AGENTS.md, or AI instruction files
- **Actual content:** The `maintaining-instruction-files` skill provides detailed guidance:
  - Size discipline: <200 lines ideal, <300 max, >300 must extract
  - Universal relevance test (applies to bug fixes AND features?)
  - Tool-first content (reference linters/skills, not rules)
  - Progressive disclosure (reference docs/, don't include content)
  - Multi-agent compatibility patterns (AGENTS.md standard)
  - Extraction workflow for oversized files
- **Impact:** Teams may bloat instruction files, include non-universal content, or fail to maintain multi-agent compatibility
- **Action:** Add section on instruction file management or explicitly reference the maintaining-instruction-files skill

### [Missing]: Learning Capture Methodology
- **Location:** Entire standards file - missing concept
- **Severity:** High
- **Current content:** No mention of capturing learning or retrospective documentation
- **Actual content:** The `capturing-learning` skill teaches:
  - When to capture (after significant work, debugging sessions, work that took longer than expected)
  - What to capture (decisions, discarded approaches, issues discovered, time notes, open questions)
  - Where to capture (CLAUDE.md for universal lessons <200 lines, docs/learning/ for detailed/edge cases)
  - Bloat warning about retrospective content in instruction files
- **Impact:** Valuable lessons learned during implementation are lost. Teams repeat same failed approaches
- **Action:** Add section on learning capture or explicitly reference the capturing-learning skill

### [Incomplete]: README Guidelines Missing Critical Details
- **Location:** "README guidelines" section (lines 40-68)
- **Severity:** Medium
- **Current content:** Lists README components but lacks depth on:
  - When to split (no criteria given)
  - How to determine "concise" vs "needs splitting"
- **Actual content:** The `organizing-documentation` skill provides:
  - Progressive disclosure by time budget (5 min: TL;DR, 20 min: README + key sections, 2 hours: full docs)
  - README-per-directory pattern (purpose statement, "use this when" section, navigation, quick links)
  - Naming conventions (ALLCAPS for document types, numeric prefixes for sequence, lowercase-dashes for content)
  - Template reference: `${CLAUDE_PLUGIN_ROOT}templates/documentation-readme-template.md`
- **Impact:** Teams may create READMEs without consistent structure or appropriate depth
- **Action:** Add progressive disclosure guidance, naming conventions, and template references

### [Incomplete]: Formatting Guidelines Missing Key Patterns
- **Location:** "Documentation formatting and structure" section (lines 14-38)
- **Severity:** Medium
- **Current content:** Basic formatting rules (headings, examples, status indicators, sentence case)
- **Actual content:** Skills teach additional patterns:
  - Maximum nesting depth (3 levels per organizing-documentation)
  - Cross-referencing between sections
  - Dual navigation (NAVIGATION.md task-based + INDEX.md concept-based)
  - Role-based reading paths (goal, reading order, time estimate, key takeaway)
  - Redirect patterns for moved content
- **Impact:** Documentation may become deeply nested or lack appropriate navigation aids
- **Action:** Add nesting limits, cross-reference guidance, and navigation patterns

### [Missing]: FIX Documentation Organization Pattern
- **Location:** Not present in standards
- **Severity:** Medium
- **Current content:** No guidance on organizing troubleshooting/debugging documentation
- **Actual content:** The `organizing-documentation` skill teaches:
  - Organize by symptoms (what developer sees), NOT by root cause
  - Example structure: symptoms/visual-bugs/, symptoms/test-failures/, symptoms/performance/
  - Clear anti-pattern: DON'T organize by root cause (memory-leaks/, type-errors/)
- **Impact:** Troubleshooting docs organized by root cause are impossible to navigate when you don't know the root cause
- **Action:** Add FIX documentation organization guidelines

### [Missing]: LOOKUP Quick Reference Standards
- **Location:** Not present in standards
- **Severity:** Medium
- **Current content:** No guidance on quick reference documentation
- **Actual content:** The `organizing-documentation` skill teaches:
  - Rule: < 30 seconds to find and use
  - Good content: keyboard shortcuts, command cheat sheets, error code tables, ID registries, one-page summaries
  - Bad content: tutorials (BUILD), explanations (UNDERSTAND), debugging guides (FIX)
- **Impact:** Teams may put lengthy tutorials or explanations in LOOKUP, defeating its purpose
- **Action:** Add LOOKUP quick reference standards with the 30-second rule

### [Outdated]: No Skill References in Standards
- **Location:** Entire document
- **Severity:** Low
- **Current content:** Standards file is self-contained with no references to skills
- **Actual content:** Each skill references related skills and standards, creating a connected knowledge base:
  - maintaining-docs-after-changes references: documentation.md, cipherpowers:maintaining-instruction-files
  - maintaining-instruction-files references: maintaining-docs-after-changes, capturing-learning, organizing-documentation
  - organizing-documentation references: maintaining-docs, maintaining-instruction-files, capturing-learning, templates
  - capturing-learning references: cipherpowers:maintaining-instruction-files, /cipherpowers:summarise
- **Impact:** Users reading standards don't know skills exist for more detailed guidance
- **Action:** Add "Related Skills" section linking to the four documentation skills

## What's Working Well

1. **Formatting guidance is accurate:** The sentence case rule, status indicators, readable formatting patterns match skill expectations

2. **README component checklist is valid:** The list of README components (getting started, dependencies, examples, troubleshooting) aligns with skill guidance

3. **README splitting guidance is directionally correct:** The advice to split large READMEs and use README_* prefix is consistent with skill patterns

4. **Definition list formatting pattern is good:** The readable formatting pattern (title on separate line from description) is a useful, specific guideline not repeated elsewhere

5. **Frontmatter metadata is correct:** The when_to_use, applies_to, related_practices, version fields follow proper practice format

6. **Consistency with documentation standard references:** The `maintaining-docs-after-changes` skill correctly references this standards file at `${CLAUDE_PLUGIN_ROOT}standards/documentation.md` (line 125)

## Additional Verification: Referenced Files Exist

| Reference (from skills) | File | Status |
|------------------------|------|--------|
| `${CLAUDE_PLUGIN_ROOT}standards/documentation.md` | plugin/standards/documentation.md | EXISTS |
| `${CLAUDE_PLUGIN_ROOT}templates/documentation-readme-template.md` | plugin/templates/documentation-readme-template.md | EXISTS |
| `${CLAUDE_PLUGIN_ROOT}standards/documentation-structure.md` | plugin/standards/documentation-structure.md | EXISTS |

All referenced files exist, so cross-references are technically valid.

## Overall Assessment

**The standards file needs significant amendment.**

The documentation.md standards file is a **minimal, legacy document** that covers only basic formatting and README structure. It predates or was never updated to incorporate the sophisticated workflows and patterns taught in the four documentation skills.

**Key gaps:**
1. No intent-based organization (the core organizing-documentation concept)
2. No maintenance workflow (the core maintaining-docs-after-changes concept)
3. No instruction file management (the core maintaining-instruction-files concept)
4. No learning capture (the core capturing-learning concept)

**Structural mismatch:**

The standards file and skills have fundamentally different scopes:
- **Standards file** (69 lines): Basic formatting + README guidelines
- **organizing-documentation** (227 lines): Complete intent-based structure, templates, anti-patterns
- **maintaining-docs-after-changes** (211 lines): Two-phase workflow, checklists, red flags
- **maintaining-instruction-files** (425 lines): Size discipline, multi-agent compatibility, extraction workflow
- **capturing-learning** (175 lines): What/when/where to capture, bloat warnings

The standards file covers perhaps 10% of what the skills teach about documentation.

**Recommended approach:**

The standards file should NOT attempt to duplicate all skill content. Instead, it should:

1. **Keep** existing formatting/structure guidance (it's accurate and not duplicated in skills)
2. **Add** intent-based structure overview (BUILD/FIX/UNDERSTAND/LOOKUP) - this is foundational
3. **Add** instruction file size limits (<200 ideal, <300 max) as a mandatory standard
4. **Add** "Related Skills" section that explicitly connects to all four documentation skills
5. **Add** brief sections that reference each skill for detailed workflows

This aligns with the progressive disclosure principle taught in maintaining-instruction-files: keep the standards file focused, reference skills for detailed workflows.

**Final verdict:** Standards file is **incomplete but not incorrect**. The content present is accurate; the problem is what's missing. Amendment needed to add intent-based structure and skill references.
