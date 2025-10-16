# Execute

Execute implementation plans with automatic agent selection, batch-level code review, and retrospective completion.

<instructions>
## Instructions

This command orchestrates plan execution in main Claude context, dispatching to specialized agents as needed:

1. **Load and analyze plan** - Read plan file, determine agent requirements
2. **Announce execution strategy** - Show which agents will handle which task types
3. **Execute in batches** - 3 tasks per batch with appropriate agents
4. **Review checkpoints** - Invoke code-reviewer after each batch
5. **Retrospective completion** - Prompt for learning capture when all tasks done

### 1. Follow the executing-plans skill

Read: `@${SUPERPOWERS_SKILLS_ROOT}/skills/collaboration/executing-plans/SKILL.md`

This skill provides:
- Plan loading and critical review
- Batch execution pattern (default 3 tasks)
- Verification requirements
- Checkpoint reporting

### 2. Analyze plan and select agents

**Before starting execution, YOU MUST:**

1. Read the entire plan file
2. Analyze each task to determine required agent
3. Announce the execution strategy to the user

**Agent selection uses hybrid approach:**

#### Keyword Analysis (Fast Path)

Scan task descriptions for these patterns:

| Keywords | Agent | Rationale |
|----------|-------|-----------|
| "Rust", ".rs file", "cargo" | rust-engineer | Rust development with TDD |
| "debug", "investigate", "production issue", "CI failure", "race condition" | ultrathink-debugger | Complex debugging |
| "review", "code review" | code-reviewer | Code review workflow |
| "retrospective", "summary", "learning" | retrospective-writer | Learning capture |
| "docs", "documentation", "README" | technical-writer | Documentation sync |

**File path analysis:**
- Tasks mentioning `.rs` files → rust-engineer
- Tasks mentioning `docs/` or `.md` files → technical-writer (if code-related), else main Claude

#### LLM Analysis (Ambiguous Cases)

When keywords don't clearly indicate an agent:

1. Read: `@plugin/skills/selecting-agents/SKILL.md`
2. Apply the decision guide to the task description
3. Consider:
   - Task complexity (simple vs complex debugging)
   - Task type (development vs documentation vs debugging)
   - Agent characteristics from skill

#### User Confirmation (Unclear Cases)

If hybrid analysis cannot confidently select an agent:

```
Task: [task description]

I cannot confidently determine which agent to use. Options:
- rust-engineer: Standard Rust development with TDD
- ultrathink-debugger: Complex debugging requiring deep investigation
- main Claude: General implementation without specialized workflow

Which agent should handle this task?
```

**Stop execution and wait for user response.**

### 3. Announce execution strategy

Before starting, announce:

```
Plan loaded: [plan file path]
Tasks: [N tasks total]

Agent assignments:
- Tasks 1-3: rust-engineer (Rust implementation)
- Tasks 4-5: main Claude (configuration updates)
- Task 6: rust-engineer (Rust testing)

Execution plan:
- Batch 1 (Tasks 1-3) → code review → fixes
- Batch 2 (Tasks 4-6) → code review → fixes
- Retrospective prompt

Ready to proceed?
```

Wait for user confirmation before starting execution.

</instructions>
