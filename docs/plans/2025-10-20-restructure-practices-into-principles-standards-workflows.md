# Restructure Practices into Principles/Standards/Workflows

> **For Claude:** REQUIRED SUB-SKILL: Use executing-plans to implement this plan task-by-task.

**Goal:** Reorganize `plugin/practices/` into three distinct directories (`principles/`, `standards/`, `workflows/`) that reflect how content is consumed.

**Architecture:** Hard cutover migration - move files to new locations and update all references in a single atomic commit. The three-way split distinguishes philosophical values (principles), concrete specifications (standards), and machine-enforceable algorithms (workflows).

**Tech Stack:** Bash, grep, find-practices tool (Rust)

---

## Task 1: Create New Directory Structure

**Files:**
- Create: `plugin/principles/`
- Create: `plugin/standards/`
- Create: `plugin/standards/rust/`
- Create: `plugin/workflows/`

**Step 1: Create new directories**

```bash
mkdir -p plugin/principles
mkdir -p plugin/standards/rust
mkdir -p plugin/workflows
```

**Step 2: Verify directory creation**

Run: `ls -la plugin/ | grep -E '(principles|standards|workflows)'`
Expected: All three directories exist with proper permissions

**Step 3: Commit directory structure**

```bash
git add plugin/principles plugin/standards plugin/workflows
git commit -m "feat: create principles/standards/workflows directory structure"
```

---

## Task 2: Move Files to New Locations

**Files:**
- Move: `plugin/practices/development.md` → `plugin/principles/development.md`
- Move: `plugin/practices/*.md` (8 files) → `plugin/standards/`
- Move: `plugin/practices/rust/*.md` (2 files) → `plugin/standards/rust/`
- Move: `plugin/practices/git-commit-algorithm.md` → `plugin/workflows/git-commit.md`

**Step 1: Move principles file**

```bash
git mv plugin/practices/development.md plugin/principles/development.md
```

**Step 2: Move standards files**

```bash
git mv plugin/practices/code-review.md plugin/standards/code-review.md
git mv plugin/practices/conventional-commits.md plugin/standards/conventional-commits.md
git mv plugin/practices/documentation.md plugin/standards/documentation.md
git mv plugin/practices/git-guidelines.md plugin/standards/git-guidelines.md
git mv plugin/practices/testing.md plugin/standards/testing.md
git mv plugin/practices/workflow.md plugin/standards/workflow.md
```

**Step 3: Move rust standards files**

```bash
git mv plugin/practices/rust/error-handling.md plugin/standards/rust/error-handling.md
git mv plugin/practices/rust/microsoft-rust-guidelines.md plugin/standards/rust/microsoft-rust-guidelines.md
```

**Step 4: Move and rename workflow file**

```bash
git mv plugin/practices/git-commit-algorithm.md plugin/workflows/git-commit.md
```

**Step 5: Verify old practices directory is empty**

Run: `ls -la plugin/practices/`
Expected: Only empty `rust/` subdirectory (will be removed in next step)

**Step 6: Remove old practices directory**

```bash
git rm -r plugin/practices/
```

**Step 7: Verify new structure**

Run: `find plugin/{principles,standards,workflows} -type f -name "*.md" | sort`
Expected: All 11 files in correct new locations

**Step 8: Commit file moves**

```bash
git add -A
git commit -m "refactor: move practices into principles/standards/workflows structure"
```

---

## Task 3: Update find-practices Tool

**Files:**
- Modify: `plugin/tools/find-practices` (bash script)

**Context:** The find-practices tool is a bash script that currently searches a single `practices/` directory. After restructuring, it needs to search three directories (`principles/`, `standards/`, `workflows/`) and aggregate results.

**Step 1: Read current find-practices script**

Read: `plugin/tools/find-practices`
Understand: Current `search_practices()` function structure

**Step 2: Update search_practices function**

Modify the script to search three directories instead of one:

Old logic (line 54):
```bash
if [ ! -d "$root/practices" ]; then
    return
fi
```

New logic:
```bash
# Search all three directory types
for dir in principles standards workflows; do
    if [ ! -d "$root/$dir" ]; then
        continue
    fi
    # Search logic here for each directory
done
```

**Step 3: Update find commands**

Update lines 60 and 78 from:
```bash
find "$root/practices" -name "*.md" -type f
```

To:
```bash
find "$root/$dir" -name "*.md" -type f
```

**Step 4: Update relative path extraction**

Update lines 66 and 84 from:
```bash
rel_path=${practice#$root/practices/}
```

To:
```bash
rel_path=${practice#$root/$dir/}
```

**Step 5: Test discovery across all three directories**

Run: `plugin/tools/find-practices "commit"`
Expected: Returns results from standards (conventional-commits.md, git-guidelines.md) and workflows (git-commit.md)

**Step 6: Test discovery for principles**

Run: `plugin/tools/find-practices "development"`
Expected: Returns results from principles (development.md)

**Step 7: Test discovery for standards**

Run: `plugin/tools/find-practices "testing"`
Expected: Returns results from standards (testing.md)

**Step 8: Test list all (no pattern)**

Run: `plugin/tools/find-practices`
Expected: Lists all practices from all three directories with proper labels

**Step 9: Commit tool updates**

```bash
git add plugin/tools/find-practices
git commit -m "feat: update find-practices to search principles/standards/workflows"
```

---

## Task 4: Update References in Commands

**Files:**
- Modify: `plugin/commands/*.md` (all command files referencing practices)

**Step 1: Find all command files with practices references**

Run: `grep -r "plugin/practices/" plugin/commands/`
Expected: List of files and line numbers with old paths

**Step 2: Update each command file**

For each file found, update references:
- `plugin/practices/code-review.md` → `plugin/standards/code-review.md`
- `plugin/practices/conventional-commits.md` → `plugin/standards/conventional-commits.md`
- `plugin/practices/git-commit-algorithm.md` → `plugin/workflows/git-commit.md`
- etc.

Use Edit tool for each file.

**Step 3: Verify no old references remain in commands**

Run: `grep -r "plugin/practices/" plugin/commands/`
Expected: No results

**Step 4: Commit command updates**

```bash
git add plugin/commands/
git commit -m "refactor: update command references to new directory structure"
```

---

## Task 5: Update References in Agents

**Files:**
- Modify: `plugin/agents/*.md` (all agent files referencing practices)

**Step 1: Find all agent files with practices references**

Run: `grep -r "plugin/practices/" plugin/agents/`
Expected: List of files and line numbers with old paths

**Step 2: Update each agent file**

For each file found, update references following same pattern as Task 4.

**Step 3: Verify no old references remain in agents**

Run: `grep -r "plugin/practices/" plugin/agents/`
Expected: No results

**Step 4: Commit agent updates**

```bash
git add plugin/agents/
git commit -m "refactor: update agent references to new directory structure"
```

---

## Task 6: Update References in Skills

**Files:**
- Modify: `plugin/skills/**/*.md` (all skill files referencing practices)

**Step 1: Find all skill files with practices references**

Run: `grep -r "plugin/practices/" plugin/skills/`
Expected: List of files and line numbers with old paths

**Step 2: Update each skill file**

For each file found, update references following same pattern as Task 4.

**Step 3: Verify no old references remain in skills**

Run: `grep -r "plugin/practices/" plugin/skills/`
Expected: No results

**Step 4: Commit skill updates**

```bash
git add plugin/skills/
git commit -m "refactor: update skill references to new directory structure"
```

---

## Task 7: Update Learning Documents

**Files:**
- Modify: `docs/learning/*.md` (retrospective files with practices references)

**Context:** Learning documents are permanent documentation (unlike plans which are ephemeral). They need to be updated to reflect the new directory structure so future readers see current paths.

**Step 1: Find all learning files with practices references**

Run: `grep -r "plugin/practices/" docs/learning/ --include="*.md" -l`
Expected: List of learning files with old paths

**Step 2: Review references in each file**

For each file found:
- Read the file to understand context
- Identify which references need updating

**Step 3: Update references in learning documents**

For each file, update references:
- `plugin/practices/code-review.md` → `plugin/standards/code-review.md`
- `plugin/practices/git-commit-algorithm.md` → `plugin/workflows/git-commit.md`
- `plugin/practices/development.md` → `plugin/principles/development.md`
- etc.

Use Edit tool for each file.

**Step 4: Verify no old references remain in learning docs**

Run: `grep -r "plugin/practices/" docs/learning/ --include="*.md"`
Expected: No results

**Step 5: Commit learning document updates**

```bash
git add docs/learning/
git commit -m "docs: update learning documents for new directory structure"
```

---

## Task 8: Update CLAUDE.md Documentation

**Files:**
- Modify: `CLAUDE.md`

**Step 1: Read current CLAUDE.md**

Read the file to understand current documentation structure.

**Step 2: Update architecture documentation**

Update the "Documentation Layer" section to reflect:
- **Principles:** Philosophical values and guidance
- **Standards:** Coding standards, conventions, guidelines
- **Workflows:** Machine-enforceable step-by-step algorithms

**Step 3: Update example references**

Change examples from:
```
@${CLAUDE_PLUGIN_ROOT}plugin/practices/code-review.md
```

To:
```
@${CLAUDE_PLUGIN_ROOT}plugin/standards/code-review.md
@${CLAUDE_PLUGIN_ROOT}plugin/principles/development.md
@${CLAUDE_PLUGIN_ROOT}plugin/workflows/git-commit.md
```

**Step 4: Update directory structure diagrams**

Update any directory tree examples to show new structure.

**Step 5: Verify no old references remain**

Run: `grep "plugin/practices/" CLAUDE.md`
Expected: No results

**Step 6: Commit CLAUDE.md updates**

```bash
git add CLAUDE.md
git commit -m "docs: update CLAUDE.md for new directory structure"
```

---

## Task 9: Update README.md Documentation

**Files:**
- Modify: `README.md`

**Step 1: Read current README.md**

Read the file to understand current documentation.

**Step 2: Update architecture documentation**

Update any references to practices directory to reflect three-way split.

**Step 3: Update examples and quick start**

Ensure examples use new paths.

**Step 4: Verify no old references remain**

Run: `grep "plugin/practices/" README.md`
Expected: No results

**Step 5: Commit README.md updates**

```bash
git add README.md
git commit -m "docs: update README.md for new directory structure"
```

---

## Task 10: Global Verification

**Files:**
- Verify: All files in repository

**Step 1: Search for any remaining old references**

Run: `grep -r "plugin/practices/" . --exclude-dir=.git --exclude-dir=target --exclude="*.md" 2>/dev/null`
Expected: No results (excluding this plan document)

**Step 2: Search in all markdown files**

Run: `grep -r "plugin/practices/" . --include="*.md" --exclude-dir=.git --exclude-dir=docs/plans 2>/dev/null`
Expected: No results (excluding plan documents)

**Step 3: Test find-practices tool comprehensively**

Run:
```bash
plugin/tools/find-practices "commit"
plugin/tools/find-practices "development"
plugin/tools/find-practices "testing"
plugin/tools/find-practices "rust"
```

Expected: All queries return appropriate results from new directories

**Step 4: Test workflow tool with new path**

Run: `plugin/tools/workflow plugin/workflows/git-commit.md --dry-run`
Expected: Workflow executes successfully

**Step 5: Verify directory structure**

Run: `tree plugin/{principles,standards,workflows} -L 2`
Expected:
```
plugin/principles/
└── development.md
plugin/standards/
├── code-review.md
├── conventional-commits.md
├── documentation.md
├── git-guidelines.md
├── testing.md
├── workflow.md
└── rust/
    ├── error-handling.md
    └── microsoft-rust-guidelines.md
plugin/workflows/
└── git-commit.md
```

**Step 6: Run full test suite (if exists)**

Run: `cargo test` or equivalent
Expected: All tests pass

---

## Task 11: Final Documentation

**Files:**
- Create: `docs/learning/2025-10-20-restructure-practices.md` (retrospective)

**Step 1: Create retrospective document**

Document the restructuring rationale, decisions made, and lessons learned following the capturing-learning skill pattern.

**Step 2: Commit retrospective**

```bash
git add docs/learning/2025-10-20-restructure-practices.md
git commit -m "docs: add retrospective for practices restructuring"
```

**Step 3: Final verification**

Run: `git log --oneline -10`
Expected: See all commits from this plan

**Step 4: Confirm migration complete**

Output summary:
- Files moved: 11
- References updated: [count from grep results]
- New directory structure: principles/standards/workflows
- Tools updated: find-practices, workflow
- Migration approach: Hard cutover (Option A)

---

## Notes

**DRY:** The Edit tool will be used repeatedly for reference updates - follow same pattern each time.

**YAGNI:** No backward compatibility layer - clean cutover.

**TDD:** No automated tests for this refactoring, but manual verification steps are comprehensive.

**Frequent commits:** One commit per task for clean history and easy rollback if needed.

**Skills referenced:**
- `@${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md` for execution
- `@${CLAUDE_PLUGIN_ROOT}/plugin/skills/documentation/capturing-learning/SKILL.md` for retrospective

---

## Deferred Items

Items deferred during code review - must be reviewed before final merge.

### From Batch 1 Review (2025-10-20-batch1-review.md)

- **[DEFERRED]** Incomplete Migration - Old Path References Remain
  - Source: Tasks 4-6 (planned for Batch 2)
  - Severity: BLOCKING
  - Reason: Out-of-scope for Batch 1 (Tasks 1-3 only). Reference updates planned for Batch 2 per original plan structure.
  - Details: 17 references to old `plugin/practices/` paths remain in commands, agents, skills, and tool README.

- **[DEFERRED]** find-practices Tool Implementation Could Be Clearer
  - Source: Task 3
  - Severity: NON-BLOCKING
  - Reason: Current implementation works correctly but has code duplication between list/search branches
  - Suggestion: Extract relative path calculation to reduce duplication

- **[DEFERRED]** Missing Verification Step in Plan Execution
  - Source: Task 3
  - Severity: NON-BLOCKING
  - Reason: Verification was done inline but not documented in commit message
  - Suggestion: Include verification evidence in future batch commit messages

- **[DEFERRED]** Documentation Addition Not Required by Batch 1
  - Source: Task 2 commit
  - Severity: NON-BLOCKING
  - Reason: Unrelated workflow syntax documentation added to refactoring commit
  - Suggestion: Separate documentation updates into commits tied to the work they document

- **[DEFERRED]** .gitkeep Files Not Removed
  - Source: Tasks 1-2
  - Severity: NON-BLOCKING
  - Reason: .gitkeep files created in Task 1 are now unnecessary after files moved in Task 2
  - Suggestion: Remove in cleanup commit
