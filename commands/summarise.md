# Summarise

Create a comprehensive retrospective summary of completed work, capturing decisions, lessons learned, and insights for continuous improvement.

<context>
## Context

**Learning Capture Methodology:**
Follow the "Capturing Learning from Completed Work" skill:
- @skills/documentation/capturing-learning/SKILL.md

**Documentation Standards:**
- @docs/practices/documentation.md

**Key Principle:**
Exhaustion after completion is when capture matters most. The harder the work, the more valuable the lessons.
</context>

<instructions>
## Instructions

Follow the systematic retrospective workflow from the capturing-learning skill:

### Step 1: Review the Work

1. **Identify work location:**
   - Check if project uses work tracking (e.g., `mise run work:active`)
   - Or determine appropriate summary location

2. **Review what was done:**
   - Run `git diff [base-branch]...HEAD` to see all changes
   - Review commit messages for key decisions
   - List approaches tried (including failed ones)
   - Note time spent vs. estimates

### Step 2: Capture Learning

Create or update summary covering:

**✅ MUST include:**
- [ ] Brief description of what was accomplished
- [ ] Key decisions made (with rationale)
- [ ] Approaches tried and discarded (what didn't work and why)
- [ ] Non-obvious issues discovered (and solutions)
- [ ] Time spent vs. estimate (if significantly different, explain)
- [ ] Things that worked well (worth repeating)
- [ ] Things that didn't work well (worth avoiding)
- [ ] Open questions or follow-up needed

**Common blind spots to check:**
- Discarded approaches (most valuable learning)
- Subtle issues that took disproportionate time
- Implicit knowledge (things you learned but didn't realize were non-obvious)

### Step 3: Save and Link

1. **Save to appropriate location:**
   - Work directory: `summary.md` (or iteration-specific)
   - No work tracking: Add to CLAUDE.md or `docs/learning/YYYY-MM-DD-topic.md`

2. **Link to implementation:**
   - Reference key files modified
   - Link to commits or PRs
   - Cross-reference CLAUDE.md if patterns emerged

3. **Make it searchable:**
   - Use descriptive headings
   - Include error messages if debugging occurred
   - Tag with relevant technology/pattern names

### Step 4: Merge if Existing

If summary already exists:
- Merge new learnings with existing content
- Ensure chronological or logical flow
- Remove redundancy

</instructions>
