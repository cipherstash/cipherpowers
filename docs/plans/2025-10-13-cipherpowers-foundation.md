# CipherPowers Foundation Implementation Plan

> **For Claude:** Use `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md` to implement this plan task-by-task.

**Goal:** Build the foundational structure for cipherpowers - a Claude Code plugin providing skills, commands, agents, and documentation for development teams.

**Architecture:** Three-layer design (Skills, Automation, Documentation) integrated with superpowers via custom find-skills tool. Plugin follows Claude Code structure with `.claude-plugin/`, `commands/`, `agents/`, `skills/`, `tools/`, and `docs/` directories.

**Tech Stack:** Bash (find-skills tool), Markdown (documentation), YAML (plugin metadata)

---

## Task 1: Plugin Metadata and Structure

**Files:**
- Create: `.claude-plugin/plugin.json`
- Create: `tools/.gitkeep`

**Step 1: Create plugin metadata**

Create `.claude-plugin/plugin.json`:

```json
{
  "name": "cipherpowers",
  "version": "0.1.0",
  "description": "Comprehensive development toolkit with skills, commands, and documentation standards",
  "author": "Your Organization",
  "repository": "https://github.com/your-org/cipherpowers"
}
```

**Step 2: Create tools directory**

```bash
mkdir -p tools
touch tools/.gitkeep
```

**Step 3: Commit plugin structure**

```bash
git add .claude-plugin/plugin.json tools/.gitkeep
git commit -m "feat: add plugin metadata and tools directory"
```

---

## Task 2: Custom find-skills Tool

**Files:**
- Create: `tools/find-skills`

**Step 1: Create find-skills script**

Create `tools/find-skills`:

```bash
#!/usr/bin/env bash
# Custom find-skills that searches both cipherpowers and superpowers

set -euo pipefail

# Determine script location to find cipherpowers root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CIPHERPOWERS_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Check for superpowers installation
if [ -z "${SUPERPOWERS_SKILLS_ROOT:-}" ]; then
    SUPERPOWERS_SKILLS_ROOT="$HOME/.config/superpowers/skills"
fi

# Parse flags
SEARCH_LOCAL=true
SEARCH_UPSTREAM=true
PATTERN=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --local)
            SEARCH_UPSTREAM=false
            shift
            ;;
        --upstream)
            SEARCH_LOCAL=false
            shift
            ;;
        *)
            PATTERN="$1"
            shift
            ;;
    esac
done

# Search function
search_skills() {
    local root=$1
    local label=$2

    if [ ! -d "$root/skills" ]; then
        return
    fi

    if [ -z "$PATTERN" ]; then
        # List all skills
        find "$root/skills" -name "SKILL.md" -type f | while read -r skill; do
            dir=$(dirname "$skill")
            name=$(basename "$dir")
            echo "[$label] $name"
        done
    else
        # Search for pattern
        find "$root/skills" -name "SKILL.md" -type f -exec grep -l -i "$PATTERN" {} \; | while read -r skill; do
            dir=$(dirname "$skill")
            name=$(basename "$dir")
            # Show path relative to skills root
            rel_path=${skill#$root/}
            echo "[$label] $rel_path"
        done
    fi
}

# Search cipherpowers skills
if [ "$SEARCH_LOCAL" = true ]; then
    search_skills "$CIPHERPOWERS_ROOT" "cipherpowers"
fi

# Search superpowers skills
if [ "$SEARCH_UPSTREAM" = true ] && [ -d "$SUPERPOWERS_SKILLS_ROOT" ]; then
    search_skills "$SUPERPOWERS_SKILLS_ROOT" "superpowers"
fi
```

**Step 2: Make script executable**

```bash
chmod +x tools/find-skills
```

**Step 3: Test find-skills**

```bash
./tools/find-skills --help || echo "Script exists and runs"
./tools/find-skills --upstream "planning"
```

Expected: Lists superpowers skills containing "planning"

**Step 4: Commit find-skills tool**

```bash
git add tools/find-skills
git commit -m "feat: add custom find-skills tool for unified discovery"
```

---

## Task 3: Extract Commands from Battlespace

**Files:**
- Create: `commands/plan.md`
- Create: `commands/work.md`
- Create: `commands/code-review.md`
- Create: `commands/commit.md`
- Create: `commands/execute.md`

**Step 1: Copy and adapt plan command**

Read `/Users/tobyhede/psrc/battlespace/.claude/commands/plan.md`, generalize for cipherpowers (remove battlespace-specific paths like `docs/work/roadmap.md`, keep the core workflow), save to `commands/plan.md`.

**Step 2: Copy and adapt work command**

Read `/Users/tobyhede/psrc/battlespace/.claude/commands/work.md`, generalize, save to `commands/work.md`.

**Step 3: Copy and adapt code-review command**

Read `/Users/tobyhede/psrc/battlespace/.claude/commands/code-review.md`, generalize, save to `commands/code-review.md`.

**Step 4: Copy and adapt commit command**

Read `/Users/tobyhede/psrc/battlespace/.claude/commands/commit.md`, generalize, save to `commands/commit.md`.

**Step 5: Copy and adapt execute command**

Read `/Users/tobyhede/psrc/battlespace/.claude/commands/execute.md`, generalize, save to `commands/execute.md`.

**Step 6: Commit commands**

```bash
git add commands/
git commit -m "feat: add extracted and generalized slash commands"
```

---

## Task 4: Extract Agents from Battlespace

**Files:**
- Create: `agents/work-planner.md`
- Create: `agents/code-reviewer.md`
- Create: `agents/work-orchestrator.md`

**Step 1: Copy and adapt work-planner agent**

Read `/Users/tobyhede/psrc/battlespace/.claude/agents/work-planner.md`, generalize (keep superpowers skill references, remove battlespace-specific doc references), save to `agents/work-planner.md`.

**Step 2: Copy and adapt code-reviewer agent**

Read `/Users/tobyhede/psrc/battlespace/.claude/agents/code-reviewer.md`, generalize, save to `agents/code-reviewer.md`.

**Step 3: Copy and adapt work-orchestrator agent**

Read `/Users/tobyhede/psrc/battlespace/.claude/agents/work-orchestrator.md`, generalize, save to `agents/work-orchestrator.md`.

**Step 4: Commit agents**

```bash
git add agents/
git commit -m "feat: add extracted and generalized agent prompts"
```

---

## Task 5: Extract and Organize Documentation

**Files:**
- Create: `docs/practices/code-review.md`
- Create: `docs/practices/conventional-commits.md`
- Create: `docs/practices/development.md`
- Create: `docs/practices/documentation.md`
- Create: `docs/practices/git-guidelines.md`
- Create: `docs/practices/testing.md`
- Create: `docs/practices/workflow.md`

**Step 1: Copy code-review practice**

Read `/Users/tobyhede/psrc/battlespace/docs/practices/code-review.md`, adapt to remove project-specific references (Linear, specific repo structure), save to `docs/practices/code-review.md`.

**Step 2: Copy conventional-commits practice**

Read `/Users/tobyhede/psrc/battlespace/docs/practices/conventional-commits.md`, generalize, save to `docs/practices/conventional-commits.md`.

**Step 3: Copy development practice**

Read `/Users/tobyhede/psrc/battlespace/docs/practices/development.md`, generalize, save to `docs/practices/development.md`.

**Step 4: Copy documentation practice**

Read `/Users/tobyhede/psrc/battlespace/docs/practices/documentation.md`, generalize, save to `docs/practices/documentation.md`.

**Step 5: Copy git-guidelines practice**

Read `/Users/tobyhede/psrc/battlespace/docs/practices/git-guidelines.md`, generalize, save to `docs/practices/git-guidelines.md`.

**Step 6: Copy testing practice**

Read `/Users/tobyhede/psrc/battlespace/docs/practices/testing.md`, generalize, save to `docs/practices/testing.md`.

**Step 7: Copy workflow practice**

Read `/Users/tobyhede/psrc/battlespace/docs/practices/workflow.md`, adapt to be more generic (keep the analysis→plan→implement→verify→review→summarize flow, but remove Linear/project-specific tooling), save to `docs/practices/workflow.md`.

**Step 8: Commit practices documentation**

```bash
git add docs/practices/
git commit -m "docs: add extracted and generalized development practices"
```

---

## Task 6: Create Skills Directory Structure

**Files:**
- Create: `skills/README.md`
- Create: `skills/.gitkeep`

**Step 1: Create skills directory**

```bash
mkdir -p skills
```

**Step 2: Create skills README**

Create `skills/README.md`:

```markdown
# CipherPowers Skills

Organization-specific skills following the superpowers framework.

## Creating Skills

See `${SUPERPOWERS_SKILLS_ROOT}/skills/meta/writing-skills/SKILL.md` for the complete process.

**Key principles:**
1. Test with subagents BEFORE writing (TDD for docs)
2. Use TodoWrite for skill creation checklist
3. Include rich `when_to_use` frontmatter
4. Follow SKILL.md structure

## Scope

Skills in this directory are:
- Organization-specific workflows
- Extensions to superpowers skills
- Universal skills under development (before upstreaming)

## Discovery

Use `tools/find-skills` to search both cipherpowers and superpowers skills.

## Structure

Each skill directory contains:
- `SKILL.md` - Main reference (required)
- Supporting files only when needed (tools, heavy reference)
```

**Step 3: Add placeholder**

```bash
touch skills/.gitkeep
```

**Step 4: Commit skills structure**

```bash
git add skills/
git commit -m "feat: add skills directory structure and documentation"
```

---

## Task 7: Create Templates Directory

**Files:**
- Create: `docs/templates/README.md`
- Create: `docs/templates/skill-template.md`

**Step 1: Create templates directory**

```bash
mkdir -p docs/templates
```

**Step 2: Create templates README**

Create `docs/templates/README.md`:

```markdown
# Templates

Reusable templates for common tasks.

## Available Templates

- `skill-template.md` - Starting point for new skills
```

**Step 3: Create skill template**

Create `docs/templates/skill-template.md`:

```markdown
---
name: Skill Name
description: One-line summary of what this does
when_to_use: Symptoms and situations when you need this
version: 1.0.0
---

# Skill Name

## Overview

What is this? Core principle in 1-2 sentences.

## When to Use

- Bullet list with SYMPTOMS and use cases
- When NOT to use

## Quick Reference

Table or bullets for scanning common operations

## Implementation

Inline code for simple patterns

## Common Mistakes

What goes wrong + fixes
```

**Step 4: Commit templates**

```bash
git add docs/templates/
git commit -m "docs: add templates directory with skill template"
```

---

## Task 8: Update Documentation Cross-References

**Files:**
- Modify: `README.md`
- Modify: `CLAUDE.md`

**Step 1: Add find-skills usage to README**

Add to README.md after "Usage" section heading:

```markdown
### Finding Skills

Use the custom find-skills tool:

\`\`\`bash
# Search all skills
tools/find-skills "pattern"

# Local only
tools/find-skills --local "pattern"

# Superpowers only
tools/find-skills --upstream "pattern"
\`\`\`
```

**Step 2: Add commands reference to README**

Replace placeholder command list with actual commands extracted.

**Step 3: Update CLAUDE.md with tool reference**

Add to CLAUDE.md after "Integration with Superpowers" section:

```markdown
## Using the find-skills Tool

The custom `tools/find-skills` script provides unified discovery:

\`\`\`bash
# From repository root
./tools/find-skills "search pattern"

# With scope flags
./tools/find-skills --local "pattern"      # cipherpowers only
./tools/find-skills --upstream "pattern"   # superpowers only
\`\`\`
```

**Step 4: Commit documentation updates**

```bash
git add README.md CLAUDE.md
git commit -m "docs: update cross-references and tool usage"
```

---

## Task 9: Create Examples Directory

**Files:**
- Create: `docs/examples/README.md`
- Create: `docs/examples/.gitkeep`

**Step 1: Create examples directory**

```bash
mkdir -p docs/examples
```

**Step 2: Create examples README**

Create `docs/examples/README.md`:

```markdown
# Examples

Real-world examples demonstrating practices and patterns.

## Structure

Examples are organized by topic:
- Commands in use
- Agent workflows
- Skill applications
- Practice implementations

Add examples as markdown files with clear naming:
- `command-plan-usage.md`
- `workflow-complete-feature.md`
- `skill-tdd-application.md`
```

**Step 3: Add placeholder**

```bash
touch docs/examples/.gitkeep
```

**Step 4: Commit examples structure**

```bash
git add docs/examples/
git commit -m "docs: add examples directory structure"
```

---

## Task 10: Verification and Final Commit

**Files:**
- Create: `.gitignore` (if not exists)

**Step 1: Create .gitignore**

Create `.gitignore`:

```
# OS files
.DS_Store
Thumbs.db

# Editor files
.vscode/
.idea/
*.swp
*.swo
*~

# Temporary files
*.tmp
*.log

# Local customization (for teams forking)
settings.local.json
```

**Step 2: Verify structure**

```bash
tree -L 2 -a
```

Expected structure:
```
.
├── .claude-plugin/
│   └── plugin.json
├── .git/
├── .gitignore
├── .vscode/
├── CLAUDE.md
├── README.md
├── agents/
│   ├── code-reviewer.md
│   ├── work-orchestrator.md
│   └── work-planner.md
├── commands/
│   ├── code-review.md
│   ├── commit.md
│   ├── execute.md
│   ├── plan.md
│   └── work.md
├── docs/
│   ├── examples/
│   ├── plans/
│   ├── practices/
│   └── templates/
├── plugins/
├── skills/
│   ├── .gitkeep
│   └── README.md
└── tools/
    └── find-skills
```

**Step 3: Test find-skills**

```bash
./tools/find-skills --upstream "brainstorm"
```

Expected: Lists brainstorming skill from superpowers

**Step 4: Commit .gitignore**

```bash
git add .gitignore
git commit -m "chore: add gitignore for common files"
```

**Step 5: Verify all commits**

```bash
git log --oneline
```

Expected: 10 commits with clear feat/docs/chore prefixes

---

## Notes

- **Linear integration:** Commands reference generic "work directory" structure. Teams using Linear should customize `commands/work.md` and `docs/practices/workflow.md` to add Linear-specific steps.

- **Rust-specific content:** Battlespace has extensive Rust/Bevy documentation. Decide whether to:
  - Include in `docs/practices/rust/` as examples
  - Create separate rust-focused cipherpowers variant
  - Leave for team customization

- **Skills migration:** After foundation is complete, analyze which battlespace workflows should become proper skills (following TDD with subagents).

- **Settings.json:** May need `.claude-plugin/settings.json` for plugin configuration. Test plugin installation to determine requirements.
