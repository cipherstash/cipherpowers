# CipherPowers for Codex

This directory contains files to enable CipherPowers skills in Codex (Anthropic's AI coding assistant).

## Files

- **cipherpowers-codex** - Executable bash script for skill discovery and loading
- **cipherpowers-bootstrap.md** - Bootstrap instructions inserted into `~/.codex/AGENTS.md`
- **INSTALL.md** - Installation guide for end users
- **README.md** - This file

## How It Works

### For Codex (No Native Plugin System)

Codex doesn't have Claude Code's native plugin system, so CipherPowers provides a CLI tool that:

1. **Lists skills** - Discovers all available skills with metadata
2. **Loads skills** - Outputs skill content for Codex to read
3. **Bootstraps** - Provides complete setup at session start

### CLI Commands

```bash
# Full bootstrap (run at session start)
~/.codex/cipherpowers/.codex/cipherpowers-codex bootstrap

# List all available skills
~/.codex/cipherpowers/.codex/cipherpowers-codex find-skills

# Load a specific skill
~/.codex/cipherpowers/.codex/cipherpowers-codex use-skill <skill-name>
```

### Skill Discovery

The script searches two locations:

1. **Personal skills** (`~/.codex/skills/`) - User-specific skills (take precedence)
2. **CipherPowers skills** (`~/.codex/cipherpowers/plugin/skills/`) - Plugin skills

Personal skills with the same name as cipherpowers skills override them.

### Skill Naming

- `cipherpowers:skill-name` - Explicitly load from cipherpowers
- `skill-name` - Load from personal first, fall back to cipherpowers

## Installation (For End Users)

See `INSTALL.md` for complete installation instructions.

Quick summary:
1. Clone to `~/.codex/cipherpowers/`
2. Add bootstrap call to `~/.codex/AGENTS.md`
3. Run `cipherpowers-codex bootstrap` to verify

## Design Decisions

### Why Bash Instead of Node.js?

- **Simplicity** - No Node.js dependency
- **Transparency** - Easy to read and modify
- **Standard tools** - Uses `awk`, `find`, `grep` (available everywhere)

### What Was Removed from Superpowers Version?

- **Git update checking** - Not needed for cipherpowers deployment model
- **Complex nested search** - CipherPowers uses flat skill structure
- **Node.js features** - Rewrote in pure bash

### Minimal Feature Set

Only three commands:
1. `bootstrap` - Full setup
2. `find-skills` - Discovery
3. `use-skill` - Loading

Everything else removed for simplicity.

## Script Architecture

### Core Functions

```bash
extract_frontmatter <file> <key>     # Parse YAML frontmatter
find_skill_file <name> <force>       # Locate skill file with override logic
print_skill_info <path> <type>       # Format skill for listing
```

### Commands

```bash
cmd_find_skills                      # List all skills
cmd_use_skill <name>                 # Load specific skill
cmd_bootstrap                        # Full bootstrap
```

### Frontmatter Parsing

Uses `awk` to extract YAML frontmatter keys:
- `name` - Skill display name
- `description` - One-line description
- `when_to_use` - Usage guidance

### Content Stripping

Removes YAML frontmatter from output (Codex doesn't need it):
- Detects `---` boundaries
- Skips frontmatter section
- Outputs only skill content

## Testing Locally

Before installation, test from project directory:

```bash
# From cipherpowers project root
.codex/cipherpowers-codex help
.codex/cipherpowers-codex find-skills
.codex/cipherpowers-codex use-skill brainstorming
```

Note: Won't find skills until installed to `~/.codex/cipherpowers/` or you modify the hardcoded paths in the script for testing.

## Differences from Claude Code Plugin

| Feature | Claude Code | Codex |
|---------|-------------|-------|
| Skill discovery | Native `Skill` tool | `cipherpowers-codex find-skills` |
| Skill loading | `Skill(skill: "name")` | `cipherpowers-codex use-skill name` |
| Session init | Automatic | Manual via `AGENTS.md` |
| Agents/Commands | Native support | Manual application only |
| Practices | Direct reference | Manual file reading |

In Codex, you only have **skills** automatically available. Agents, commands, and practices can be referenced manually but aren't automated.
