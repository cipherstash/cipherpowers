# CipherPowers Bootstrap for Codex

<EXTREMELY_IMPORTANT>
You have cipherpowers.

**Tool for running skills:**
- `~/.codex/cipherpowers/.codex/cipherpowers-codex use-skill <skill-name>`

**Tool Mapping for Codex:**
When skills reference tools you don't have, substitute your equivalent tools:
- `TodoWrite` → `update_plan` (your planning/task tracking tool)
- `Task` tool with subagents → Tell the user that subagents aren't available in Codex yet and you'll do the work the subagent would do
- `Skill` tool → `~/.codex/cipherpowers/.codex/cipherpowers-codex use-skill` command (already available)
- `Read`, `Write`, `Edit`, `Bash` → Use your native tools with similar functions
- `mcp__serena__*` tools → Use your equivalent file search/symbol navigation tools

**Skills naming:**
- CipherPowers skills: `cipherpowers:skill-name` (from ~/.codex/cipherpowers/plugin/skills/)
- Personal skills: `skill-name` (from ~/.codex/skills/)
- Personal skills override cipherpowers skills when names match

**Critical Rules:**
- Before ANY task, review the skills list (shown below)
- If a relevant skill exists, you MUST use `~/.codex/cipherpowers/.codex/cipherpowers-codex use-skill` to load it
- Announce: "I've read the [Skill Name] skill and I'm using it to [purpose]"
- Skills with checklists require `update_plan` todos for each item
- NEVER skip mandatory workflows (brainstorming before coding, TDD, systematic debugging)

**Skills location:**
- CipherPowers skills: ~/.codex/cipherpowers/plugin/skills/
- Personal skills: ~/.codex/skills/ (override cipherpowers when names match)

**CipherPowers Plugin Architecture:**
CipherPowers provides three layers:
1. **Skills** (plugin/skills/) - Reusable workflows and techniques
2. **Automation** (plugin/agents/, plugin/commands/) - Specialized agents and commands
3. **Documentation** (plugin/standards/, plugin/principles/) - Coding standards and practices

Note: In Codex, you only have direct access to skills. Agent and command patterns can be applied manually.

IF A SKILL APPLIES TO YOUR TASK, YOU DO NOT HAVE A CHOICE. YOU MUST USE IT.
</EXTREMELY_IMPORTANT>
