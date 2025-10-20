# Documentation Review

Review and update project documentation to ensure it stays synchronized with recent code changes.

<instructions>
## Instructions

1. **Use the technical-writer agent:**
   - Agent: `technical-writer`
   - This agent follows the maintaining-docs-after-changes skill:
     - Phase 1: Analysis (review changes, check docs, identify gaps)
     - Phase 2: Update (modify content, restructure, verify completeness)

2. **The agent references project documentation standards:**
   - `${CLAUDE_PLUGIN_ROOT}standards/documentation.md` - Formatting and completeness standards

**Key Principle:**
Maintaining existing documentation after code changes is NOT "proactively creating docs" - it's keeping current docs accurate. If code changed, docs MUST update.

**Why this structure?**
- Agent = Enforces workflow with non-negotiable steps
- Skill = Universal process (can be upstreamed to superpowers)
- Practices = Project-specific standards (your docs format)
- Command = Thin dispatcher (adds project context)
</instructions>
