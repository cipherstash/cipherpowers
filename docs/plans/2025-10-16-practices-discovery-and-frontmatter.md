# Practices Discovery and Frontmatter Implementation Plan

> **For Claude:** Use `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md` to implement this plan task-by-task.

**Goal:** Enable discovery and referencing of practices through a find-practices tool, environment variable support, and standardized frontmatter for documentation standards.

**Architecture:** Create practices directory structure with templates, build a `find-practices` discovery tool parallel to `find-skills`, add YAML frontmatter to all practice documents, and establish the `${CIPHERPOWERS_ROOT}` environment variable pattern for direct references.

**Tech Stack:** Bash scripting, YAML frontmatter, markdown documentation

---

## Task 1: Create Practices Directory Structure

**Files:**
- Create: `docs/practices/`
- Create: `tools/`

**Step 1: Create directories**

Run: `mkdir -p docs/practices tools`
Expected: Directories created successfully

**Step 2: Verify structure**

Run: `ls -la docs/ && ls -la tools/`
Expected: Shows practices directory under docs/ and tools/ directory

**Step 3: Commit**

```bash
git add docs/practices tools
git commit -m "chore: create practices and tools directories

Foundation for practice documentation and discovery tools."
```

---

## Task 2: Create Practice Template

**Files:**
- Create: `docs/practices/_template.md`

**Step 1: Write practice template**

Create `docs/practices/_template.md`:

```markdown
---
name: Practice Name
description: Brief statement of goal/principle - 1-2 sentences
when_to_use: when [specific condition or scenario requiring this practice]
applies_to: [Optional: languages, frameworks, or contexts - e.g., "Rust", "Python", "all projects"]
related_practices: [Optional: comma-separated list of related practice filenames]
version: 1.0.0
---

# Practice Name

[Brief statement of goal/principle - 1-2 sentences]

## Standards

[Universal standards - what quality looks like]

### [Category 1]

[Standards for this category]

### [Category 2]

[Standards for this category]

## Project Configuration

[Project-specific tooling, commands, and conventions]

### Commands

**[Task type]:**
- Command: `[specific command]`
- Requirements: [what must pass]

**[Another task type]:**
- Command: `[specific command]`
- Requirements: [what must pass]

### File Conventions

**[File type]:**
- Location: `[path/pattern]`
- Format: `[naming convention]`
- Template: [if applicable]

### Tool Configuration

[Any tool-specific settings or requirements]

## Checklist

[What to verify - references both standards and project config]

- [ ] [Check 1 - references standard]
- [ ] [Check 2 - references project config]
- [ ] [Check 3 - combines both]

## References

[Links to external specifications, if applicable]
- [Spec name]: [URL or file reference]
```

**Step 2: Verify template format**

Run: `head -30 docs/practices/_template.md`
Expected: Shows frontmatter and template structure

**Step 3: Commit**

```bash
git add docs/practices/_template.md
git commit -m "docs: create practice template with frontmatter

Define standard structure for practices:
- YAML frontmatter (name, description, when_to_use, etc.)
- Standards section (universal principles)
- Project Configuration section (commands, conventions)
- Checklist and References sections"
```

---

## Task 3: Create find-practices Discovery Tool

**Files:**
- Create: `tools/find-practices`

**Step 1: Write find-practices script**

Create `tools/find-practices`:

```bash
#!/usr/bin/env bash
# Custom find-practices that searches cipherpowers and marketplace practices

set -euo pipefail

# Determine script location to find cipherpowers root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CIPHERPOWERS_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

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

# Extract frontmatter field from a practice file
extract_field() {
    local file=$1
    local field=$2

    # Extract value from YAML frontmatter
    awk -v field="$field" '
        /^---$/ { in_fm = !in_fm; next }
        in_fm && $1 == field":" {
            sub(/^[^:]+:[[:space:]]*/, "")
            print
            exit
        }
    ' "$file"
}

# Search function
search_practices() {
    local root=$1
    local label=$2

    if [ ! -d "$root/docs/practices" ]; then
        return
    fi

    if [ -z "$PATTERN" ]; then
        # List all practices with metadata
        find "$root/docs/practices" -name "*.md" -type f ! -name "_template.md" | sort | while read -r practice; do
            name=$(extract_field "$practice" "name")
            description=$(extract_field "$practice" "description")
            when_to_use=$(extract_field "$practice" "when_to_use")

            # Show path relative to docs/practices
            rel_path=${practice#$root/docs/practices/}

            if [ -n "$when_to_use" ]; then
                echo "Use /$rel_path when $when_to_use"
            elif [ -n "$description" ]; then
                echo "[$label] $rel_path - $description"
            else
                echo "[$label] $rel_path"
            fi
        done
    else
        # Search for pattern in practices (frontmatter + content)
        find "$root/docs/practices" -name "*.md" -type f ! -name "_template.md" -exec grep -l -i "$PATTERN" {} \; | sort | while read -r practice; do
            name=$(extract_field "$practice" "name")
            description=$(extract_field "$practice" "description")

            # Show path relative to docs/practices
            rel_path=${practice#$root/docs/practices/}

            if [ -n "$description" ]; then
                echo "[$label] $rel_path - $description"
            else
                echo "[$label] $rel_path"
            fi
        done
    fi
}

# Search cipherpowers practices
if [ "$SEARCH_LOCAL" = true ]; then
    search_practices "$CIPHERPOWERS_ROOT" "cipherpowers"
fi

# Search marketplace practices (if available)
if [ "$SEARCH_UPSTREAM" = true ] && [ -n "${CIPHERPOWERS_MARKETPLACE_ROOT:-}" ] && [ -d "$CIPHERPOWERS_MARKETPLACE_ROOT" ]; then
    search_practices "$CIPHERPOWERS_MARKETPLACE_ROOT" "marketplace"
fi
```

**Step 2: Make script executable**

Run: `chmod +x tools/find-practices`
Expected: No output, script is now executable

**Step 3: Test basic execution (no practices yet)**

Run: `./tools/find-practices`
Expected: No output (no practices exist yet besides template)

**Step 4: Commit**

```bash
git add tools/find-practices
git commit -m "feat: add find-practices discovery tool

Discovery tool for practices with metadata support:
- Searches docs/practices/ directory
- Extracts frontmatter (name, description, when_to_use)
- Supports --local and --upstream flags
- Parallel to find-skills architecture"
```

---

## Task 4: Create Sample Code Review Practice

**Files:**
- Create: `docs/practices/code-review.md`

**Step 1: Write code-review practice**

Create `docs/practices/code-review.md`:

```markdown
---
name: Code Review
description: Standards for conducting thorough, constructive code reviews with severity-based feedback
when_to_use: when reviewing code changes, before merging pull requests, or establishing review processes
applies_to: all projects
related_practices: testing.md, development.md
version: 1.0.0
---

# Code Review

Standards for conducting thorough, constructive code reviews with severity-based feedback.

## Standards

### Review Depth

**All reviews must verify:**
- Functionality: Code does what it claims
- Tests: Adequate coverage and quality
- Design: Appropriate abstractions and patterns
- Security: No vulnerabilities or unsafe practices
- Performance: No obvious bottlenecks
- Maintainability: Clear, documented, follows conventions

### Feedback Severity Levels

**CRITICAL** - Must fix before merge
- Security vulnerabilities
- Data loss risks
- Breaking changes without migration
- Test failures

**MAJOR** - Should fix before merge
- Design flaws
- Missing error handling
- Inadequate test coverage
- Performance issues

**MINOR** - Nice to have
- Style inconsistencies
- Documentation improvements
- Refactoring opportunities

**QUESTION** - Seeking clarification
- Understanding intent
- Exploring alternatives
- Learning context

## Project Configuration

### Commands

**Run tests:**
- Command: `mise run test`
- Requirements: All tests must pass

**Run quality checks:**
- Command: `mise run check`
- Requirements: All checks must pass

### File Conventions

**Code review reports:**
- Location: `work/<branch-name>/code-review-*.md`
- Format: `code-review-YYYY-MM-DD-HHMM.md`
- Template: Structured feedback with severity markers

### Tool Configuration

Use git commands to identify changes:
- `git diff main...HEAD` - All changes in branch
- `git log main..HEAD` - Commit history

## Checklist

- [ ] All tests pass (mise run test)
- [ ] All quality checks pass (mise run check)
- [ ] Code matches requirements
- [ ] Tests cover new functionality
- [ ] No CRITICAL or MAJOR issues unresolved
- [ ] Feedback saved to work directory

## References

- Skill: `skills/conducting-code-review/SKILL.md`
- Upstream: `${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/requesting-code-review/SKILL.md`
```

**Step 2: Verify practice created**

Run: `head -20 docs/practices/code-review.md`
Expected: Shows frontmatter and beginning of practice

**Step 3: Test find-practices with new practice**

Run: `./tools/find-practices`
Expected: Shows code-review.md with metadata

**Step 4: Test pattern search**

Run: `./tools/find-practices "review"`
Expected: Returns code-review.md

**Step 5: Commit**

```bash
git add docs/practices/code-review.md
git commit -m "docs: add code review practice

Define standards for code reviews:
- Review depth requirements
- Severity-based feedback levels
- Project commands and conventions
- Integration with code-review skill"
```

---

## Task 5: Update Plugin Configuration

**Files:**
- Modify: `.claude-plugin/plugin.json`

**Step 1: Read current plugin.json**

Run: `cat .claude-plugin/plugin.json`
Expected: Shows current plugin configuration

**Step 2: Add environment variable**

Modify `.claude-plugin/plugin.json` to add environment section after repository field:

```json
{
  "name": "cipherpowers",
  "version": "0.1.0",
  "description": "Comprehensive development toolkit with skills, commands, and documentation standards",
  "author": "Your Organization",
  "repository": "https://github.com/your-org/cipherpowers",
  "environment": {
    "CIPHERPOWERS_ROOT": "${PLUGIN_DIR}"
  },
  "hooks": {
    "SessionStart": [
      {
        "type": "skill",
        "path": "./skills/using-skills/SKILL.md"
      }
    ]
  }
}
```

**Step 3: Validate JSON syntax**

Run: `python3 -m json.tool .claude-plugin/plugin.json > /dev/null && echo "Valid JSON"`
Expected: "Valid JSON"

**Step 4: Commit**

```bash
git add .claude-plugin/plugin.json
git commit -m "feat: add CIPHERPOWERS_ROOT environment variable

Set CIPHERPOWERS_ROOT to plugin directory for:
- Direct practice references in agents/commands
- find-practices tool discovery
- Consistent path resolution"
```

---

## Task 6: Document Environment Variables and Tools

**Files:**
- Modify: `CLAUDE.md:131-155`

**Step 1: Add environment variables section**

In `CLAUDE.md`, add after line 87 (after "Skill references:" section):

```markdown

## Environment Variables

**CIPHERPOWERS_ROOT**: Path to the cipherpowers plugin installation
- Set automatically when plugin is loaded
- Use in agents/commands for direct practice references: `@${CIPHERPOWERS_ROOT}/docs/practices/name.md`
- Also used by `find-practices` tool for discovery

**CIPHERPOWERS_MARKETPLACE_ROOT**: (Optional) Path to marketplace installation for shared practices
- Set if using cipherpowers as a local marketplace
- Enables `--upstream` flag in `find-practices`
```

**Step 2: Update discovery tools section**

Replace "Using the find-skills Tool" section (around line 89) with:

```markdown
## Using the Discovery Tools

**find-skills**: Discover available skills

```bash
# From repository root
./tools/find-skills "search pattern"

# With scope flags
./tools/find-skills --local "pattern"      # cipherpowers only
./tools/find-skills --upstream "pattern"   # superpowers only
```

**find-practices**: Discover available practices

```bash
# From repository root
./tools/find-practices "search pattern"

# With scope flags
./tools/find-practices --local "pattern"      # cipherpowers only
./tools/find-practices --upstream "pattern"   # marketplace only
```

**Direct references**: When you know the practice name

```markdown
# In agents or commands
Read: @${CIPHERPOWERS_ROOT}/docs/practices/code-review.md
```
```

**Step 3: Verify documentation is clear**

Run: `grep -A 10 "CIPHERPOWERS_ROOT" CLAUDE.md | head -15`
Expected: Shows new environment variable documentation

**Step 4: Commit**

```bash
git add CLAUDE.md
git commit -m "docs: document CIPHERPOWERS_ROOT and find-practices

Add documentation for:
- CIPHERPOWERS_ROOT environment variable
- CIPHERPOWERS_MARKETPLACE_ROOT for marketplace support
- find-practices tool usage and examples
- Direct practice reference pattern"
```

---

## Task 7: Create Example Agent Using Practices

**Files:**
- Create: `agents/example-reviewer.md`

**Step 1: Write example agent**

Create `agents/example-reviewer.md`:

```markdown
---
name: example-reviewer
description: Example agent demonstrating practice references
color: blue
---

You are an example agent showing how to reference practices.

## Context

YOU MUST READ THESE FILES IN ORDER:

1. **Project Practice** (standards):
   - Code Review Standards: @${CIPHERPOWERS_ROOT}/docs/practices/code-review.md

2. **Project Context**:
   - README.md: @README.md
   - Architecture: @CLAUDE.md

## Workflow

1. Read the code-review practice to understand severity levels
2. Apply those standards to your review
3. Reference the practice in your feedback

This demonstrates:
- Using ${CIPHERPOWERS_ROOT} for direct references
- Practices define WHAT quality looks like
- Agents reference practices, not duplicate them
```

**Step 2: Verify agent can reference practice**

Run: `grep "CIPHERPOWERS_ROOT" agents/example-reviewer.md`
Expected: Shows practice reference pattern

**Step 3: Commit**

```bash
git add agents/example-reviewer.md
git commit -m "docs: add example agent with practice references

Demonstrate how agents reference practices:
- Use ${CIPHERPOWERS_ROOT} variable
- Direct @ syntax for file references
- Practices provide standards, agents apply them"
```

---

## Task 8: Add find-practices to README

**Files:**
- Modify: `README.md` (assuming it exists)

**Step 1: Check if README exists**

Run: `test -f README.md && echo "exists" || echo "create it"`
Expected: Either "exists" or "create it"

**Step 2: Add tools section to README**

If README exists, add section about discovery tools. If not, create minimal README:

```markdown
# CipherPowers

Comprehensive development toolkit for Claude Code.

## Discovery Tools

**Find Skills:**
```bash
./tools/find-skills "pattern"
./tools/find-skills --local "pattern"   # cipherpowers only
./tools/find-skills --upstream "pattern" # superpowers only
```

**Find Practices:**
```bash
./tools/find-practices "pattern"
./tools/find-practices --local "pattern"   # cipherpowers only
./tools/find-practices --upstream "pattern" # marketplace only
```

## Direct References

In agents and commands, use:
- `@${CIPHERPOWERS_ROOT}/docs/practices/practice-name.md` - Direct practice reference
- `@${SUPERPOWERS_SKILLS_ROOT}/skills/category/skill-name/SKILL.md` - Skill reference

## Documentation

See `CLAUDE.md` for complete architecture and usage guide.
```

**Step 3: Verify README format**

Run: `head -30 README.md`
Expected: Shows tools documentation

**Step 4: Commit**

```bash
git add README.md
git commit -m "docs: document discovery tools in README

Add quick reference for:
- find-skills and find-practices usage
- Direct reference patterns
- Link to detailed documentation"
```

---

## Task 9: Test Complete Workflow

**Files:**
- N/A (testing)

**Step 1: Test finding practices**

Run: `./tools/find-practices`
Expected: Lists code-review.md with "when to use" metadata

**Step 2: Test pattern search**

Run: `./tools/find-practices "code"`
Expected: Returns code-review.md with description

**Step 3: Test --local flag**

Run: `./tools/find-practices --local "review"`
Expected: Returns code-review.md

**Step 4: Verify practice frontmatter parses**

Run: `head -10 docs/practices/code-review.md`
Expected: Shows valid YAML frontmatter

**Step 5: Verify environment variable pattern**

Run: `grep -r "CIPHERPOWERS_ROOT" agents/ docs/ README.md`
Expected: Shows usage in example-reviewer.md, CLAUDE.md, and README.md

No commit (testing only)

---

## Completion Checklist

After completing all tasks, verify:

- [ ] docs/practices/ directory exists with template
- [ ] tools/find-practices script is executable
- [ ] find-practices can discover practices with frontmatter
- [ ] CIPHERPOWERS_ROOT documented in CLAUDE.md
- [ ] Environment variable added to plugin.json
- [ ] Example practice (code-review.md) created
- [ ] Example agent demonstrates practice references
- [ ] README documents discovery tools
- [ ] All commits follow conventional format
- [ ] Pattern search works (./tools/find-practices "pattern")
- [ ] Direct reference pattern documented for agents
