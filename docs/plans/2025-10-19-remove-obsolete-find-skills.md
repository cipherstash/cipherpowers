# Remove Obsolete find-skills System Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use executing-plans to implement this plan task-by-task.

**Goal:** Remove obsolete bash script discovery system and update documentation to reflect Claude Code's native skills system.

**Architecture:** Claude Code now provides native skill discovery through the Skill tool. The custom bash `find-skills` script and related `using-skills` wrapper are obsolete. The `find-practices` tool remains useful for practices discovery since practices aren't skills.

**Tech Stack:** Documentation updates (Markdown), file deletions, bash script preservation (find-practices only)

---

## Task 1: Delete Obsolete using-skills Skill

**Files:**
- Delete: `plugin/skills/using-skills/SKILL.md`

**Rationale:** This skill only provided tool paths for the obsolete bash `find-skills` script and architecture overview already in CLAUDE.md. The upstream superpowers `using-skills` provides all necessary workflow discipline.

**Step 1: Verify the file only contains obsolete content**

```bash
cat plugin/skills/using-skills/SKILL.md
```

Expected: File contains references to `./tools/find-skills` bash script and architecture overview

**Step 2: Delete the file**

```bash
rm plugin/skills/using-skills/SKILL.md
```

**Step 3: Delete the directory if empty**

```bash
rmdir plugin/skills/using-skills
```

**Step 4: Verify deletion**

```bash
ls -la plugin/skills/ | grep using-skills
```

Expected: No output (directory removed)

**Step 5: Commit**

```bash
git add -A plugin/skills/using-skills
git commit -m "refactor: remove obsolete using-skills wrapper

The cipherpowers using-skills skill only provided paths to the obsolete
bash find-skills script. Claude Code's native Skill tool has replaced
bash script discovery. The upstream superpowers using-skills provides
all necessary workflow discipline."
```

---

## Task 2: Delete Obsolete find-skills Script

**Files:**
- Delete: `plugin/tools/find-skills`

**Rationale:** Claude Code's native Skill tool automatically discovers skills in `plugin/skills/` directories. The bash script discovery mechanism is obsolete.

**Step 1: Verify the script is for skill discovery**

```bash
head -20 plugin/tools/find-skills
```

Expected: Script searches `skills/` directories in cipherpowers and superpowers

**Step 2: Delete the script**

```bash
rm plugin/tools/find-skills
```

**Step 3: Verify find-practices remains**

```bash
ls -la plugin/tools/
```

Expected: `find-practices` script still exists

**Step 4: Commit**

```bash
git add plugin/tools/find-skills
git commit -m "refactor: remove obsolete find-skills script

Claude Code's native Skill tool provides automatic skill discovery.
The bash script approach is no longer needed. Keeping find-practices
as it remains useful for practices discovery."
```

---

## Task 3: Update plugin/skills/README.md

**Files:**
- Modify: `plugin/skills/README.md:22-24`

**Step 1: Read current content**

```bash
cat plugin/skills/README.md
```

**Step 2: Update the Discovery section**

Replace lines 22-24:

```markdown
## Discovery

Use `tools/find-skills` to search both cipherpowers and superpowers skills.
```

With:

```markdown
## Discovery

Skills are automatically discovered by Claude Code's native Skill tool. All skills in `plugin/skills/` are available in Claude Code sessions.

To use a skill in conversation, Claude uses the `Skill` tool with the skill name. For example:
- `Skill(command: "cipherpowers:conducting-code-review")`
- `Skill(command: "superpowers:brainstorming")`

Skills from both cipherpowers and upstream superpowers are available seamlessly.
```

**Step 3: Verify the change**

```bash
cat plugin/skills/README.md
```

Expected: Discovery section explains native Claude Code skill discovery

**Step 4: Commit**

```bash
git add plugin/skills/README.md
git commit -m "docs: update skills README for native skill discovery

Replaced bash script discovery instructions with explanation of
Claude Code's native Skill tool."
```

---

## Task 4: Update CLAUDE.md - Remove Discovery Tools Section

**Files:**
- Modify: `CLAUDE.md:203-252`

**Step 1: Read current Integration with Superpowers section**

```bash
sed -n '203,252p' CLAUDE.md
```

Expected: Section contains bash script references for `find-skills` and `find-practices`

**Step 2: Replace the "Integration with Superpowers" section**

Replace lines 203-252 with:

```markdown
## Integration with Superpowers

CipherPowers integrates seamlessly with the superpowers plugin through Claude Code's native skills system.

**Skill Discovery:**
- Skills from both plugins are automatically discovered
- Use the Skill tool in conversations: `Skill(command: "plugin-name:skill-name")`
- No manual discovery scripts needed

**Practices Discovery:**
Custom `find-practices` tool (`${CLAUDE_PLUGIN_ROOT}tools/find-practices`):
- Searches `${CLAUDE_PLUGIN_ROOT}plugin/practices/` (local practices)
- Searches `${CIPHERPOWERS_MARKETPLACE_ROOT}/plugin/practices/` (marketplace practices, if available)
- Extracts YAML frontmatter (name, description, when_to_use)
- Flags: `--local`, `--upstream`, or default (both)

**Usage:**
```bash
# Find practices
./plugin/tools/find-practices "pattern"
./plugin/tools/find-practices --local "pattern"    # cipherpowers only
./plugin/tools/find-practices --upstream "pattern" # marketplace only
```

**Direct References:**
Commands and agents reference skills and practices using environment variables:
- `@${CLAUDE_PLUGIN_ROOT}plugin/practices/practice-name.md` - Direct practice reference
- `@${SUPERPOWERS_SKILLS_ROOT}/skills/category/skill-name/SKILL.md` - Upstream skill reference
- Skills are invoked via Skill tool, not direct file references
```

**Step 3: Verify the change**

```bash
sed -n '203,270p' CLAUDE.md
```

Expected: New, concise section focusing on native skill discovery and find-practices

**Step 4: Commit**

```bash
git add CLAUDE.md
git commit -m "docs: update CLAUDE.md for native skill discovery

Removed obsolete bash script discovery documentation. Simplified
Integration with Superpowers section to focus on native Skill tool
and the still-useful find-practices tool."
```

---

## Task 5: Update CLAUDE.md - Remove Working with Skills Section

**Files:**
- Modify: `CLAUDE.md:254-261`

**Step 1: Read current "Working with Skills in this Repository" section**

```bash
sed -n '254,261p' CLAUDE.md
```

Expected: Section references meta-skill and TDD approach

**Step 2: Update the section**

Replace lines 254-261 with:

```markdown
## Working with Skills in this Repository

When creating or editing skills in `plugin/skills/`:

1. **Read the meta-skill:** `${SUPERPOWERS_SKILLS_ROOT}/skills/meta/writing-skills/SKILL.md`
2. **Follow TDD:** Test with subagents BEFORE writing
3. **Use TodoWrite:** Create todos for the skill creation checklist
4. **Consider upstream:** Universal skills may be contributed to superpowers later
5. **Skills are auto-discovered:** Once created in `plugin/skills/`, they're automatically available via the Skill tool
```

**Step 3: Verify the change**

```bash
sed -n '254,262p' CLAUDE.md
```

Expected: Section includes note about automatic discovery

**Step 4: Commit**

```bash
git add CLAUDE.md
git commit -m "docs: add auto-discovery note to skills section

Clarified that skills in plugin/skills/ are automatically discovered
by Claude Code's native system."
```

---

## Task 6: Update README.md - Remove Discovery Tools Section

**Files:**
- Modify: `README.md:129-152`

**Step 1: Read current Discovery Tools and Direct References sections**

```bash
sed -n '129,152p' README.md
```

Expected: Sections document bash scripts for find-skills and find-practices

**Step 2: Replace both sections**

Replace lines 129-152 with:

```markdown
## Skills and Practices

**Skills:** Automatically discovered by Claude Code. All skills in `plugin/skills/` are available via the Skill tool.

**Practices:** Use the find-practices tool to discover available practices:

```bash
./plugin/tools/find-practices "pattern"
./plugin/tools/find-practices --local "pattern"    # cipherpowers only
./plugin/tools/find-practices --upstream "pattern" # marketplace only
```

**Direct references in agents/commands:**
- `@${CLAUDE_PLUGIN_ROOT}plugin/practices/practice-name.md` - Direct practice reference
- `@${SUPERPOWERS_SKILLS_ROOT}/skills/category/skill-name/SKILL.md` - Upstream skill reference
```

**Step 3: Verify the change**

```bash
sed -n '129,148p' README.md
```

Expected: Concise section focusing on practices discovery and direct references

**Step 4: Commit**

```bash
git add README.md
git commit -m "docs: simplify README discovery section

Removed obsolete find-skills documentation. Kept find-practices
usage as it remains useful. Clarified skills are auto-discovered."
```

---

## Task 7: Update plugin/commands/commit.md

**Files:**
- Modify: `plugin/commands/commit.md:21`

**Step 1: Read current Why this structure section**

```bash
sed -n '20,24p' plugin/commands/commit.md
```

Expected: Line 21 says "Skill = Discoverable workflow (agents can find it with find-skills)"

**Step 2: Update line 21**

Replace:
```markdown
- Skill = Discoverable workflow (agents can find it with find-skills)
```

With:
```markdown
- Skill = Discoverable workflow (automatically available via Skill tool)
```

**Step 3: Verify the change**

```bash
sed -n '20,24p' plugin/commands/commit.md
```

Expected: Updated explanation without find-skills reference

**Step 4: Commit**

```bash
git add plugin/commands/commit.md
git commit -m "docs: update commit command skill discovery note

Replaced find-skills reference with native Skill tool explanation."
```

---

## Task 8: Verify No Remaining References

**Files:**
- Check: All markdown files in repository

**Step 1: Search for find-skills references in plugin/**

```bash
grep -r "find-skills" plugin/ --include="*.md" || echo "No references found"
```

Expected: No references found (or only in historical docs/plans which are fine)

**Step 2: Search for using-skills references in plugin/**

```bash
grep -r "using-skills" plugin/ --include="*.md" || echo "No references found"
```

Expected: No references found in plugin/ directory

**Step 3: Search for obsolete tool path references**

```bash
grep -r "\./tools/find-skills" . --include="*.md" | grep -v "^docs/plans" || echo "No active references found"
```

Expected: Only historical references in docs/plans (which document past decisions)

**Step 4: List remaining tools**

```bash
ls -la plugin/tools/
```

Expected: find-practices present, find-skills absent

---

## Task 9: Update Session Start Hook (if needed)

**Files:**
- Check: `plugin/hooks/session-start.sh` (if it exists and references find-skills)

**Step 1: Check if hook exists and references find-skills**

```bash
if [ -f plugin/hooks/session-start.sh ]; then
  grep -n "find-skills" plugin/hooks/session-start.sh || echo "No reference found"
else
  echo "Hook does not exist"
fi
```

**Step 2: If references found, update them**

If the hook provides skill discovery info in session start, remove find-skills references and explain native discovery instead.

**Step 3: Commit if changed**

```bash
if git diff --quiet plugin/hooks/session-start.sh; then
  echo "No changes needed"
else
  git add plugin/hooks/session-start.sh
  git commit -m "refactor: remove find-skills from session hook

Updated to explain native skill discovery instead of bash script."
fi
```

---

## Task 10: Final Verification and Documentation

**Files:**
- Create: `docs/learning/2025-10-19-remove-find-skills.md` (optional retrospective)

**Step 1: Run final verification**

```bash
echo "=== Deleted Files ==="
ls plugin/skills/using-skills/ 2>&1 || echo "✓ using-skills/ removed"
ls plugin/tools/find-skills 2>&1 || echo "✓ find-skills removed"

echo -e "\n=== Remaining Tools ==="
ls plugin/tools/

echo -e "\n=== Plugin References Check ==="
grep -r "find-skills\|using-skills" plugin/ --include="*.md" | wc -l
echo "references found (should be 0)"
```

Expected:
- using-skills/ directory: removed
- find-skills script: removed
- find-practices: still present
- 0 references to obsolete tools in plugin/

**Step 2: Test that practices discovery still works**

```bash
./plugin/tools/find-practices "commit"
```

Expected: Lists commit-related practices

**Step 3: Review all commits**

```bash
git log --oneline -10
```

Expected: Series of focused commits documenting the removal

**Step 4: Create final summary commit (if needed)**

Only if there are small doc fixes needed:

```bash
git add -A
git commit -m "docs: final cleanup of find-skills references"
```

---

## Notes for Engineer

**Why we're keeping find-practices:**
- Practices are NOT skills - they're standards documents with YAML frontmatter
- Claude Code's Skill tool only discovers skills (SKILL.md files)
- The find-practices script provides useful discovery for practices
- When Claude Code adds native practices support, we can remove find-practices too

**Historical references are fine:**
- Files in `docs/plans/` and `docs/learning/` document past decisions
- Don't update these - they're historical records
- Only update active documentation (README, CLAUDE.md, plugin/)

**Testing approach:**
- This is primarily a deletion + documentation task
- No unit tests needed
- Verification is manual: check files deleted, search for references
- Functional test: Verify find-practices still works after changes
