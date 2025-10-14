# Documentation Review

Review and update project documentation to ensure it stays synchronized with recent code changes.

<context>
## Context

**Documentation Methodology:**
Follow the "Maintaining Documentation After Code Changes" skill:
- @skills/documentation/maintaining-docs-after-changes/SKILL.md

**Documentation Standards:**
- @docs/practices/documentation.md

**Key Principle:**
Maintaining existing documentation after code changes is NOT "proactively creating docs" - it's keeping current docs accurate. If code changed, docs MUST update.
</context>

<instructions>
## Instructions

Follow the two-phase workflow from the documentation maintenance skill:

### Phase 1: Analysis

1. **Review recent changes:**
   - Run `git diff [base-branch]...HEAD` to see full scope of changes
   - Examine all modified files and understand implementation

2. **Check existing documentation systematically:**
   - @README.md
   - @CLAUDE.md
   - All files matching `README_*.md`
   - @docs/practices/ directory
   - @docs/examples/ directory
   - Any other project-specific documentation

3. **Identify documentation gaps using the skill's checklist:**
   - New features without usage examples
   - Changed APIs without updated references
   - Implementation lessons not captured in CLAUDE.md
   - New best practices discovered during work
   - Implementation challenges and solutions
   - Known issues or limitations discovered
   - Commands/tasks that changed behavior
   - Troubleshooting insights from debugging

**Output of Phase 1:** Specific list of documentation updates needed

### Phase 2: Update

1. **Update content to reflect changes:**
   - Add new sections for new features
   - Update changed functionality (especially examples)
   - Document implementation details for complex components
   - Update usage examples with new functionality
   - Add troubleshooting tips from recent issues
   - Update best practices in CLAUDE.md
   - Verify commands/tasks reflect current behavior

2. **Restructure for clarity:**
   - Remove outdated or redundant content
   - Group related topics together
   - Split large files if needed
   - Fix broken cross-references
   - Ensure consistent formatting

3. **Provide summary of documentation updates:**
   - Files updated
   - Major documentation changes
   - New best practices documented
   - Sections removed or restructured

### Critical Checks

**MUST verify all items from skill's "What NOT to Skip" checklist:**
- [ ] git diff reviewed (full diff, not summary)
- [ ] CLAUDE.md updated (best practices, lessons learned)
- [ ] All README*.md files checked and updated
- [ ] docs/practices/ updated if patterns changed
- [ ] docs/examples/ updated if usage changed
- [ ] Usage examples updated (not just prose)
- [ ] Troubleshooting sections updated
- [ ] Cross-references verified

**Common blind spots to check explicitly:**
- CLAUDE.md lessons learned (consistently missed)
- Usage examples in docs/examples/
- Troubleshooting documentation
- Practices documentation updates

</instructions>
