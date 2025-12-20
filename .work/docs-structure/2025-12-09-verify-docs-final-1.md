# Documentation Standards Verification Report #1

## Metadata
- Date: 2025-12-09
- Reviewer: Agent #1
- Subject: plugin/standards/documentation.md
- Ground truth: Four documentation skills
  - plugin/skills/organizing-documentation/SKILL.md
  - plugin/skills/maintaining-docs-after-changes/SKILL.md
  - plugin/skills/maintaining-instruction-files/SKILL.md
  - plugin/skills/capturing-learning/SKILL.md

## Executive Summary
- Total issues found: 12
- Critical: 2, High: 4, Medium: 4, Low: 2

The documentation standards file (`plugin/standards/documentation.md`) is relatively lightweight at 69 lines, covering basic formatting and README guidelines. However, it is significantly **incomplete** compared to what the four documentation skills teach. The standards file functions more as a basic style guide rather than a comprehensive documentation practices standard. Many critical concepts from the skills are entirely absent.

## Issues Found

### [MISSING]: Intent-Based Documentation Structure (BUILD/FIX/UNDERSTAND/LOOKUP)
- **Location:** Entire standards file
- **Severity:** CRITICAL
- **Current content:** The standards file has no mention of the intent-based documentation structure.
- **Actual content:** The `organizing-documentation` skill teaches a comprehensive intent-based structure:
  - BUILD/ (00-START, 01-DESIGN, 02-IMPLEMENT, 03-TEST, 04-VERIFY)
  - FIX/ (symptoms, investigation, solutions)
  - UNDERSTAND/ (core-systems, evolution)
  - LOOKUP/ (quick references)
  - Specific rules: FIX organized by symptoms not causes, LOOKUP items < 30 seconds
- **Impact:** Teams using only the standards file will miss the core organizational principle for documentation. This is foundational to the documentation approach.
- **Action:** Add a new major section on "Documentation Organization" that explains the intent-based structure with references to the organizing-documentation skill.

### [MISSING]: Documentation Maintenance Workflow
- **Location:** Entire standards file
- **Severity:** CRITICAL
- **Current content:** No mention of when or how to maintain documentation after code changes.
- **Actual content:** The `maintaining-docs-after-changes` skill provides:
  - Two-phase workflow: Analyze (git diff, identify gaps) then Update (sync, restructure)
  - Critical principle: "If you changed the code, you must update the corresponding documentation. No exceptions."
  - Detailed checklist of what to check every time
  - Common rationalizations and why they're wrong
  - Red flags that indicate documentation needs attention
- **Impact:** Without maintenance guidance, documentation drifts and becomes stale. The standards file provides formatting rules but no workflow for keeping docs current.
- **Action:** Add a section on "Documentation Maintenance" with key principles and reference to the maintaining-docs-after-changes skill.

### [MISSING]: Instruction File Guidelines (CLAUDE.md/AGENTS.md)
- **Location:** Entire standards file
- **Severity:** HIGH
- **Current content:** No mention of instruction files, their constraints, or multi-agent compatibility.
- **Actual content:** The `maintaining-instruction-files` skill teaches:
  - Size constraints: <200 lines ideal, <300 max
  - Progressive disclosure via docs/ references
  - Multi-agent neutral language
  - Tool-first content approach
  - Extraction workflow when files exceed limits
  - AGENTS.md vs CLAUDE.md patterns
- **Impact:** Instruction files are critical infrastructure that shape every AI interaction. Without guidance, they become bloated and ineffective.
- **Action:** Add a section on "Instruction File Standards" covering size limits, progressive disclosure, and multi-agent compatibility, with reference to the skill.

### [MISSING]: Learning Capture Guidelines
- **Location:** Entire standards file
- **Severity:** HIGH
- **Current content:** No mention of capturing learning or retrospectives.
- **Actual content:** The `capturing-learning` skill teaches:
  - When to capture learning (after complex work, debugging sessions, etc.)
  - What to capture (decisions, discarded approaches, issues discovered, time notes)
  - Where to capture (CLAUDE.md for universal, docs/learning/ for specific)
  - Minimal structure for learning documentation
- **Impact:** Valuable lessons from completed work are lost without systematic capture. This is a documentation practice that should be referenced.
- **Action:** Add a brief section on "Capturing Learning" with reference to the capturing-learning skill.

### [MISSING]: INDEX.md Requirements
- **Location:** Entire standards file
- **Severity:** HIGH
- **Current content:** No mention of index files or navigation aids.
- **Actual content:** The `organizing-documentation` skill specifies:
  - INDEX.md must have purpose column (mandatory)
  - Dual navigation: NAVIGATION.md (task-based) + INDEX.md (concept-based)
  - README-per-directory pattern
- **Impact:** Documentation discoverability suffers without proper indexing and navigation.
- **Action:** Add guidance on index files and navigation patterns.

### [MISSING]: Progressive Disclosure Pattern
- **Location:** Entire standards file
- **Severity:** HIGH
- **Current content:** No mention of progressive disclosure.
- **Actual content:** Multiple skills teach this:
  - `maintaining-instruction-files`: "Reference, don't include" principle
  - `organizing-documentation`: Multiple entry points by time budget (5 min TL;DR, 20 min README + key sections, 2 hours full docs)
- **Impact:** Documentation without progressive disclosure overwhelms readers and wastes context window space.
- **Action:** Add a section on progressive disclosure with time-budget guidance.

### [INCOMPLETE]: README Guidelines Missing Key Elements
- **Location:** "README guidelines" section (lines 40-69)
- **Severity:** MEDIUM
- **Current content:** Lists what README should include: short paragraph, getting started, dependencies, task reference, examples, troubleshooting, links, project structure, tech stack, license.
- **Actual content:** The `organizing-documentation` skill adds:
  - README-per-directory pattern (every directory needs README.md)
  - Required README structure: Purpose statement, "Use this when" section, Navigation to contents, Quick links to related sections
  - Template reference: `${CLAUDE_PLUGIN_ROOT}templates/documentation-readme-template.md`
- **Impact:** README guidance is present but incomplete. Directory-level READMEs are not covered.
- **Action:** Expand README guidelines to include directory-level README requirements and link to the template.

### [MISSING]: Naming Conventions for Documentation
- **Location:** Entire standards file
- **Severity:** MEDIUM
- **Current content:** Only mentions README_*.md prefix convention.
- **Actual content:** The `organizing-documentation` skill specifies:
  - ALLCAPS for document types: SUMMARY.md, QUICK-REFERENCE.md
  - Numeric prefixes for sequence: 00-START/, 01-DESIGN/
  - Lowercase-dashes for content: api-patterns.md
- **Impact:** Inconsistent naming without clear conventions.
- **Action:** Add a "Naming Conventions" section.

### [MISSING]: Anti-Patterns Section
- **Location:** Entire standards file
- **Severity:** MEDIUM
- **Current content:** No explicit anti-patterns or "what not to do" guidance.
- **Actual content:** The `organizing-documentation` skill has explicit anti-patterns:
  - Don't create deep nesting (max 3 levels)
  - Don't duplicate content across directories
  - Don't put tutorials in LOOKUP
  - Don't organize FIX by root cause
  - Don't skip INDEX.md
- **Impact:** Without anti-patterns, common mistakes are repeated.
- **Action:** Add an anti-patterns section.

### [INCOMPLETE]: Status Indicators Usage
- **Location:** "Documentation formatting and structure" section (line 17)
- **Severity:** MEDIUM
- **Current content:** Mentions "Use status indicators (checkmark, warning, X) consistently" but provides no guidance on when to use each.
- **Actual content:** The `maintaining-instruction-files` skill uses specific status indicators:
  - Checkmark Good: <200 lines
  - Warning: 200-300 lines
  - X Action required: >300 lines
- **Impact:** Readers don't know how to apply status indicators consistently.
- **Action:** Add examples of when to use each status indicator.

### [OUTDATED]: Reference to Non-Existent Standards File
- **Location:** Implicit in the skill references
- **Severity:** LOW
- **Current content:** The standards file exists at `plugin/standards/documentation.md`.
- **Actual content:** The `organizing-documentation` skill references `${CLAUDE_PLUGIN_ROOT}standards/documentation-structure.md` which appears to be a different file.
- **Impact:** Unclear which standards file is authoritative or if there should be two separate files.
- **Action:** Verify whether documentation-structure.md exists or if the reference should be to documentation.md.

### [INCOMPLETE]: Related Skills/Practices Cross-References
- **Location:** YAML frontmatter (line 6)
- **Severity:** LOW
- **Current content:** Only lists `related_practices: development.md`
- **Actual content:** Should reference all four documentation skills as related practices/skills.
- **Impact:** Discoverability of related documentation workflow is limited.
- **Action:** Add references to the four documentation skills in frontmatter or a new "Related Skills" section.

## What's Working Well

1. **Basic formatting guidance is solid** - The formatting section (headings, code examples, status indicators, Markdown formatting, sentence case) provides clear, actionable guidance.

2. **Definition list pattern** - The example of separating titles from descriptions with proper formatting is useful and specific.

3. **README splitting guidance** - The advice to split large README files into README_*.md files with cross-references is aligned with the skills' approach of keeping main files concise.

4. **Frontmatter structure** - Using YAML frontmatter with name, description, when_to_use, applies_to, and version follows good practices.

5. **Concise format** - At 69 lines, the standards file itself follows the principle of being concise, though this comes at the cost of completeness.

## Overall Assessment

The `plugin/standards/documentation.md` file **requires significant amendment**. It currently functions as a basic formatting style guide rather than a comprehensive documentation practices standard.

**Key gaps:**
1. No coverage of the intent-based documentation structure (BUILD/FIX/UNDERSTAND/LOOKUP) - this is foundational
2. No maintenance workflow or "when to update" guidance
3. No instruction file (CLAUDE.md/AGENTS.md) specific guidance
4. No learning capture guidance
5. No navigation/indexing requirements

**Recommendation:** The standards file should either:
- **Option A:** Expand significantly to cover all documentation practices, adding approximately 100-150 lines of content covering the missing areas
- **Option B:** Remain a focused "formatting standards" file but add a prominent section linking to the four skills for comprehensive documentation workflows

Given the current structure and that the skills exist with detailed guidance, **Option B** may be more appropriate - keep the standards file focused on universal formatting rules but add clear references to the skills for workflow guidance. This would follow the "progressive disclosure" principle the skills themselves teach.

The standards file should explicitly state its scope (formatting and README structure) and direct readers to the skills for:
- Documentation organization (organizing-documentation)
- Documentation maintenance workflow (maintaining-docs-after-changes)
- Instruction file management (maintaining-instruction-files)
- Learning capture (capturing-learning)

---

## Appendix: Line-by-Line Mapping

### Standards File Section: "Documentation formatting and structure" (lines 12-38)

| Standards Content | Skill Verification | Status |
|------------------|-------------------|--------|
| Clear headings and sections | Aligned with organizing-documentation | OK |
| Include code examples | Aligned with maintaining-docs-after-changes | OK |
| Status indicators (checkmark, warning, X) | maintaining-instruction-files uses these but standards doesn't explain when | INCOMPLETE |
| Proper Markdown formatting | Universal good practice | OK |
| Sentence case for titles | No skill mentions this | UNIQUE TO STANDARDS |
| Cover all implemented features | Aligned with maintaining-docs-after-changes | OK |
| Include usage examples | Aligned with maintaining-docs-after-changes | OK |
| Document API changes | Aligned with maintaining-docs-after-changes | OK |
| Troubleshooting guidance | maintaining-docs-after-changes: "Add troubleshooting tips from recent issues" | OK |
| Definition list formatting example | No skill mentions this specific pattern | UNIQUE TO STANDARDS |

### Standards File Section: "README guidelines" (lines 40-69)

| Standards Content | Skill Verification | Status |
|------------------|-------------------|--------|
| Short paragraph for what/why | Not explicitly in skills | UNIQUE TO STANDARDS |
| Getting started | organizing-documentation: BUILD/00-START/ | ALIGNED |
| Essential dependencies | organizing-documentation: prerequisites | ALIGNED |
| Task/command reference | organizing-documentation: LOOKUP/ | ALIGNED |
| Code usage examples | maintaining-docs-after-changes emphasis | ALIGNED |
| Troubleshooting | organizing-documentation: FIX/ | ALIGNED |
| Links/cross-references | organizing-documentation: cross-references | ALIGNED |
| Project structure | organizing-documentation: structure | ALIGNED |
| Tech stack | Not in skills | UNIQUE TO STANDARDS |
| License | Not in skills | UNIQUE TO STANDARDS |
| Split large READMEs | organizing-documentation: README-per-directory | ALIGNED |
| README_*.md naming | organizing-documentation prefers directory READMEs | DIVERGENT |

### Missing from Standards (per Skills)

| Skill | Key Concept | Standards Coverage |
|-------|-------------|-------------------|
| organizing-documentation | BUILD/FIX/UNDERSTAND/LOOKUP structure | MISSING |
| organizing-documentation | FIX organized by symptoms | MISSING |
| organizing-documentation | LOOKUP < 30 seconds | MISSING |
| organizing-documentation | INDEX.md with purpose column | MISSING |
| organizing-documentation | NAVIGATION.md dual navigation | MISSING |
| organizing-documentation | Naming conventions (ALLCAPS, numeric, lowercase-dashes) | MISSING |
| organizing-documentation | Anti-patterns list | MISSING |
| organizing-documentation | Progressive disclosure time budgets | MISSING |
| organizing-documentation | Directory README requirements | MISSING |
| organizing-documentation | Redirect pattern for moved content | MISSING |
| maintaining-docs-after-changes | Two-phase workflow | MISSING |
| maintaining-docs-after-changes | "If you changed code, update docs" principle | MISSING |
| maintaining-docs-after-changes | Checklist of what to verify | MISSING |
| maintaining-docs-after-changes | Common rationalizations table | MISSING |
| maintaining-docs-after-changes | Red flags list | MISSING |
| maintaining-instruction-files | <200/300 line limits | MISSING |
| maintaining-instruction-files | Progressive disclosure principle | MISSING |
| maintaining-instruction-files | Tool-first content | MISSING |
| maintaining-instruction-files | Multi-agent neutral language | MISSING |
| maintaining-instruction-files | Extraction workflow | MISSING |
| maintaining-instruction-files | AGENTS.md standard | MISSING |
| capturing-learning | When to capture | MISSING |
| capturing-learning | What to capture structure | MISSING |
| capturing-learning | Where to capture decision tree | MISSING |
| capturing-learning | Common rationalizations | MISSING |
