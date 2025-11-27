# CipherPowers Plugin Architecture - Exploration Summary

## Overview

CipherPowers is a Claude Code plugin providing a comprehensive toolkit for development teams. It's structured as a **three-layer plugin architecture** that separates concerns: Skills (reusable workflows), Automation (commands and agents), and Documentation (practices and standards).

## 1. Plugin Configuration Structure

### marketplace.json Location and Format

**Location:** `.claude-plugin/marketplace.json` (at project root level, not in plugin directory)

**Structure:**
```json
{
  "name": "cipherpowers",
  "owner": {
    "name": "Organization Name",
    "email": "email@example.com"
  },
  "metadata": {
    "description": "Plugin description",
    "version": "0.1.0"
  },
  "plugins": [
    {
      "name": "cipherpowers",
      "source": "./plugin",
      "description": "Full description",
      "version": "0.1.0"
    }
  ]
}
```

**Key Point:** `"source": "./plugin"` means the plugin root points to the `./plugin/` directory. This is critical for understanding environment variable resolution.

### Environment Variable: CLAUDE_PLUGIN_ROOT

- Claude Code sets `${CLAUDE_PLUGIN_ROOT}` automatically when plugin loads
- Points to the directory specified in `marketplace.json` `"source"` field
- In cipherpowers: points to `/plugin/` directory (relative to project root)
- Used throughout agent markdown files with `@` syntax: `@${CLAUDE_PLUGIN_ROOT}standards/code-review.md`
- **CRITICAL:** Relative paths like `@skills/...` do NOT work in subagent contexts (confirmed via testing)
- Must use full variable syntax: `@${CLAUDE_PLUGIN_ROOT}skills/conducting-code-review/SKILL.md`

## 2. Plugin Directory Structure

```
cipherpowers/
├── .claude-plugin/
│   └── marketplace.json              # Plugin registration metadata
├── plugin/
│   ├── commands/                     # Slash commands (/.cipherpowers:xxx)
│   │   ├── brainstorm.md
│   │   ├── plan.md
│   │   ├── execute.md
│   │   ├── code-review.md
│   │   ├── commit.md
│   │   ├── verify.md
│   │   └── summarise.md
│   │
│   ├── agents/                       # Specialized subagent prompts
│   │   ├── code-review-agent.md
│   │   ├── rust-agent.md
│   │   ├── ultrathink-debugger.md
│   │   ├── technical-writer.md
│   │   ├── plan-review-agent.md
│   │   ├── code-agent.md
│   │   ├── commit-agent.md
│   │   ├── gatekeeper.md
│   │   ├── execute-review-agent.md
│   │   ├── review-collation-agent.md
│   │   ├── research-agent.md
│   │   └── path-test-agent.md
│   │
│   ├── skills/                       # Organization-specific reusable workflows
│   │   ├── conducting-code-review/
│   │   │   ├── SKILL.md
│   │   │   └── test-scenarios.md
│   │   ├── test-driven-development/
│   │   ├── executing-plans/
│   │   ├── writing-plans/
│   │   ├── verifying-plans/
│   │   ├── brainstorming/
│   │   ├── capturing-learning/
│   │   ├── commit-workflow/
│   │   ├── dual-verification/
│   │   ├── systematic-debugging/
│   │   ├── using-cipherpowers/
│   │   ├── algorithmic-command-enforcement/
│   │   └── [24+ more skills]
│   │
│   ├── hooks/                        # Quality enforcement system
│   │   ├── hooks.json                # Hook registration (routes to CLI)
│   │   ├── gates.json                # Plugin default gates configuration
│   │   ├── context/                  # Plugin-level context files
│   │   │   └── session-start.md      # Injects on SessionStart
│   │   ├── hooks-app/                # TypeScript application
│   │   │   ├── src/
│   │   │   │   ├── cli.ts            # Entry point
│   │   │   │   ├── dispatcher.ts
│   │   │   │   ├── context.ts
│   │   │   │   ├── config.ts
│   │   │   │   ├── gates/            # TypeScript gate implementations
│   │   │   │   │   ├── index.ts
│   │   │   │   │   ├── commands.ts
│   │   │   │   │   ├── plan-compliance.ts
│   │   │   │   │   └── plugin-path.ts
│   │   │   │   └── types.ts
│   │   │   └── dist/                 # Compiled JavaScript
│   │   ├── examples/                 # Example configurations
│   │   │   ├── context/
│   │   │   ├── strict.json
│   │   │   ├── permissive.json
│   │   │   └── pipeline.json
│   │   ├── README.md
│   │   ├── ARCHITECTURE.md
│   │   ├── SETUP.md
│   │   ├── CONVENTIONS.md
│   │   └── TYPESCRIPT.md
│   │
│   ├── standards/                    # Project-specific conventions & practices
│   │   ├── code-review.md            # Code review standards & severity levels
│   │   ├── conventional-commits.md   # Commit message format
│   │   ├── git-guidelines.md
│   │   ├── documentation.md
│   │   ├── logging.md
│   │   └── rust/                     # Language-specific standards
│   │
│   ├── principles/                   # Universal development philosophies
│   │   ├── development.md            # Core: Simple, Consistent, Documented
│   │   └── testing.md
│   │
│   ├── templates/                    # Templates for creating new components
│   │   ├── agent-template.md         # Agent structure with persuasion principles
│   │   ├── skill-template.md
│   │   ├── practice-template.md
│   │   ├── code-review-template.md   # Code review report structure
│   │   ├── code-review-request.md
│   │   ├── verify-template.md        # Verification review structure
│   │   ├── verify-plan-template.md
│   │   ├── verify-collation-template.md
│   │   └── README.md
│   │
│   ├── context/                      # Plugin-level context injection (fallback defaults)
│   │   └── session-start.md
│   │
│   ├── examples/                     # Example documentation
│   │   └── README.md
│   │
│   ├── docs/                         # Configuration & integration docs
│   │   ├── configuring-project-commands.md
│   │   └── [other integration docs]
│   │
│   └── .claude-plugin/               # Plugin metadata (mirrors root)
│       └── marketplace.json
│
├── CLAUDE.md                         # Project instructions for Claude
├── docs/                             # Project documentation (NOT shipped with plugin)
└── [other project files]
```

**Key Distinction:**
- `/plugin/` - All content shipped with the plugin to users
- `/docs/` - Project documentation about building cipherpowers (NOT shipped)
- `/plugin/principles/` - Universal philosophies (all projects)
- `/plugin/standards/` - Project-specific conventions (customizable)

## 3. How Hooks Work

### hooks.json Registration

**Location:** `plugin/hooks/hooks.json`

**Purpose:** Routes Claude Code hook events to the TypeScript CLI application

**Format:**
```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": ".*",
        "hooks": [
          {
            "type": "command",
            "command": "node ${CLAUDE_PLUGIN_ROOT}/hooks/hooks-app/dist/cli.js"
          }
        ]
      }
    ],
    "SubagentStop": [
      {
        "matcher": ".*",
        "hooks": [
          {
            "type": "command",
            "command": "node ${CLAUDE_PLUGIN_ROOT}/hooks/hooks-app/dist/cli.js"
          }
        ]
      }
    ],
    "UserPromptSubmit": [...],
    "SessionStart": [...]
  }
}
```

**How it works:**
1. Claude Code fires hook event (e.g., `PostToolUse`)
2. `hooks.json` matches event and calls registered command
3. Command invokes TypeScript CLI: `node ${CLAUDE_PLUGIN_ROOT}/hooks/hooks-app/dist/cli.js`
4. CLI handles all hook logic (context injection, gate execution, actions)

**Supported Events:**
- SessionStart / SessionEnd
- UserPromptSubmit
- SlashCommandStart / SlashCommandEnd
- SkillStart / SkillEnd
- SubagentStop
- PreToolUse / PostToolUse
- Stop
- Notification

### gates.json Configuration

**Location:** `plugin/hooks/gates.json` (plugin defaults)

**Override Location:** `.claude/gates.json` (project-specific overrides)

**Self-Referential Design:**
- Plugin uses its own `gates.json` to configure default behaviors
- TypeScript gates have no `command` field (implemented in `src/gates/`)
- Shell command gates have `command` field (execute shell commands)

**Configuration Format:**
```json
{
  "gates": {
    "check": {
      "description": "Run project quality checks",
      "keywords": ["lint", "check", "format", "quality"],
      "command": "npm run lint",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    },
    "test": {
      "description": "Run project test suite",
      "keywords": ["test", "testing", "spec"],
      "command": "npm test",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    },
    "plan-compliance": {
      "description": "Verify work follows active plan",
      "on_pass": "CONTINUE",
      "on_fail": "BLOCK"
    }
  },
  "hooks": {
    "UserPromptSubmit": {
      "gates": ["check", "test", "build"]
    },
    "PostToolUse": {
      "enabled_tools": ["Edit", "Write"],
      "gates": ["check"]
    },
    "SubagentStop": {
      "enabled_agents": ["rust-agent", "code-review-agent"],
      "gates": ["check", "test"]
    }
  }
}
```

**Key Features:**
- **Keyword Triggering:** Gates only run when relevant keywords found in user message
- **Gate Chaining:** Gates can chain to other gates (subroutine calls)
- **Configuration Merging:** Project `.claude/gates.json` overrides plugin defaults
- **Tool/Agent Filtering:** Can restrict gates to specific tools or agents

### Context Injection

**Zero-config content injection** via file naming convention.

**File Pattern:** `.claude/context/{name}-{stage}.md`

**Examples:**
```
.claude/context/code-review-start.md          # /cipherpowers:code-review starts
.claude/context/test-driven-development-start.md  # TDD skill loads
.claude/context/session-start.md              # SessionStart fires
.claude/context/slash-command/execute-start.md    # /cipherpowers:execute starts
.claude/context/skill/capturing-learning-start.md # Skill loads
```

**Priority Order:**
1. **Project context** (`.claude/context/`) - highest priority
2. **Plugin context** (`${CLAUDE_PLUGIN_ROOT}/context/`) - fallback defaults

## 4. Command Structure

Commands are **thin dispatchers** that:
- Reference skills for methodology
- Reference practices for project configuration
- Dispatch to specialized agents or main Claude context
- Do NOT contain workflow logic (that's in agents)

**Example: execute.md**
```markdown
# Execute

Execute implementation plans with automatic agent selection...

## Algorithmic Workflow

**Decision tree (follow exactly):**
1. Is this a plan execution request?
2. Does a plan exist?
3. Load and activate skill: cipherpowers:executing-plans
4. Follow skill exactly
5. Stop when complete
```

**Pattern:**
- Algorithmic decision trees for 100% compliance
- Skill activation mandatory (via Skill tool)
- Non-negotiable workflow enforcement

## 5. Agent Structure

Agents contain **complete workflows** with persuasion principles.

**Template Pattern (4 persuasion principles):**
1. **Authority Principle** - Imperative language, non-negotiable steps
2. **Commitment Principle** - Announcements and checklists create accountability
3. **Scarcity Principle** - Immediate requirements and completion criteria
4. **Social Proof Principle** - Failure modes and rationalization defenses

**Example: code-review-agent.md**
```markdown
---
name: code-review-agent
description: Meticulous principal engineer who reviews code
color: red
---

You are a meticulous, pragmatic principal engineer...

## MANDATORY: Skill Activation
Load skill: @${CLAUDE_PLUGIN_ROOT}skills/conducting-code-review/SKILL.md
Step 1 - EVALUATE: YES/NO
Step 2 - ACTIVATE: Use Skill tool

## Non-Negotiable Workflow
- Announcement (Commitment)
- Pre-work checklist
- Core workflow steps
- Completion criteria
- Handling bypass requests

## Red Flags - STOP (Social Proof)
- If thinking ANY of these, violating workflow
- Table of rationalizations and corrections
```

## 6. Skill Structure

Skills are **reusable process knowledge** written following TDD.

**Location:** `plugin/skills/{skill-name}/`

**Contents:**
- `SKILL.md` - Main skill documentation
- `test-scenarios.md` - Pressure test scenarios (optional)

**SKILL.md Format:**
```markdown
---
name: Skill Name
description: What it does
when_to_use: When to invoke it
version: X.Y.Z
---

# Skill Name

## Overview
[Purpose and quick reference]

## Implementation
[Step-by-step workflow with prerequisites]

### Step-by-Step Workflow

#### 1. First Step
[Details, commands, validation]

#### 2. Second Step
[Details, commands, validation]

## What NOT to Skip
[Critical steps and common rationalizations]

## Related Skills
[Cross references]

## Testing This Skill
[References test-scenarios.md]
```

**Auto-Discovery:**
- All skills in `plugin/skills/` automatically discovered
- Invoked via Skill tool: `Skill(skill: "cipherpowers:skill-name")`
- No manual registration needed

## 7. Standards & Practices Structure

**Location:** `plugin/standards/{name}.md`

**Format:**
```markdown
---
name: Practice Name
description: Brief description
when_to_use: When to apply
applies_to: All projects / Specific languages
related_practices: other-practice.md
version: 1.0.0
---

# Practice Name

## Core Principles
[Universal principles - what quality looks like]

## Review Checklist & Severity
[BLOCKING vs NON-BLOCKING categories]

## Project Configuration
[Project-specific settings and commands]
```

**Pattern:**
- **Universal Standards** - Same across all projects
- **Project Configuration** - Customizable per project
- Agents/Skills reference practices (not vice versa)

## 8. Plugin Discovery & Integration

### How Claude Code Finds Plugin
1. User installs plugin via CLI
2. Plugin system reads `.claude-plugin/marketplace.json`
3. Routes slash commands to plugin agents
4. Registers hooks from `plugin/hooks/hooks.json`
5. Makes skills discoverable automatically

### File Path References

**In Markdown Agent Files:**
- Use `${CLAUDE_PLUGIN_ROOT}` with `@` syntax
- `@${CLAUDE_PLUGIN_ROOT}skills/conducting-code-review/SKILL.md` ✅ WORKS
- `@skills/conducting-code-review/SKILL.md` ❌ FAILS in subagents
- `${CLAUDE_PLUGIN_ROOT}/hooks/gates.json` ✅ WORKS (in JSON configs)

**Why:**
- Variable expands correctly when agents invoked as subagents
- Relative paths without variable don't resolve in subagent contexts
- Consistent with marketplace.json `"source": "./plugin/"` setting

## 9. Architecture Principles

### DRY (Don't Repeat Yourself)
- Standards live in one place
- Skills reference practices
- Commands reference skills
- Changes propagate automatically

### SRP (Single Responsibility)
- **Practices** = WHAT to follow
- **Skills** = HOW to do it
- **Commands** = WHEN to invoke
- **Agents** = WHO enforces workflow

### Testability
- Skills include TDD test scenarios
- Baseline tests prove problems exist
- With-skill tests verify effectiveness under pressure
- Test scenarios document expected violations

### Reusability
- Skills are universal (portable across projects/teams)
- Practices are project-specific (customized for team)
- Commands add project context to universal skills

## 10. Key Takeaways for Plugin Development

1. **marketplace.json** must be at `.claude-plugin/marketplace.json`
2. **Plugin root** points to directory in `source` field (typically `./plugin/`)
3. **Environment variable** `${CLAUDE_PLUGIN_ROOT}` set automatically
4. **Always use** `@${CLAUDE_PLUGIN_ROOT}...` in agent markdown files
5. **hooks.json** registers all event types → single TypeScript CLI entry point
6. **gates.json** uses plugin's own config format (self-referential design)
7. **Context injection** auto-discovers files via naming convention
8. **Commands** are thin dispatchers → skill activation mandatory
9. **Agents** are thick with workflows → non-negotiable process + persuasion principles
10. **Skills** are reusable → TDD-first, auto-discovered via Skill tool

## 11. Configuration Merging Pattern

```
plugin/hooks/gates.json     (plugin defaults)
        ↓ merged with
.claude/gates.json          (project overrides)
        ↓
Merged Configuration        (project takes precedence)
```

**Behavior:**
- Plugin provides baseline configuration
- Project configuration overrides at key level
- Enables zero-config setup with customization

## Summary: Creating turboshovel Plugin

For a new plugin to follow cipherpowers patterns:

1. Create `.claude-plugin/marketplace.json` pointing to `./plugin/`
2. Organize plugin content in `/plugin/` directory
3. Register hooks in `plugin/hooks/hooks.json`
4. Provide default gates in `plugin/hooks/gates.json`
5. Use `${CLAUDE_PLUGIN_ROOT}` variable in all agent file references
6. Create thin command dispatchers that reference skills
7. Create thick agents with non-negotiable workflows
8. Create reusable skills with TDD scenarios
9. Separate universal standards from project config
10. Enable context injection via `.claude/context/` file naming
