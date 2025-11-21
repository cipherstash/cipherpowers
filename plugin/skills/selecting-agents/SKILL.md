---
name: Selecting Agents
description: Decision guide for choosing the right specialized agent for each task type
when_to_use: before dispatching work to specialized agents, when multiple agents could apply
version: 1.0.0
---

# Selecting Agents

## Overview

Use the right agent for the job. Each agent is optimized for specific scenarios and follows a focused workflow.

**This skill helps you choose** which specialized agent to use based on the task at hand.

**For automatic agent selection:** When executing implementation plans, use the `/execute` command which applies this skill's logic automatically with hybrid keyword/LLM analysis. Manual selection using this skill is for ad-hoc agent dispatch outside of plan execution.

## Agent Selection Logic

When selecting agents (manually or automatically), you must analyze the **task requirements and context**, not just match keywords naively.

**DO NOT use naive keyword matching:**
- ❌ Task contains "ultrathink" → select ultrathink-debugger
- ❌ Task contains "rust" → select rust-engineer
- ❌ Task mentions agent name → select that agent

**DO use semantic understanding:**
- ✅ Analyze what the task is asking for (debugging? implementation? review?)
- ✅ Consider task complexity and characteristics
- ✅ Match agent capabilities to task requirements
- ✅ Ignore mentions of agent names that are not prescriptive

**Examples of INCORRECT selection:**
- Task: "Fix simple bug (don't use ultrathink-debugger, it's overkill)" → ❌ Selecting ultrathink-debugger because "ultrathink" appears
- Task: "Implement feature X in Python (not Rust)" → ❌ Selecting rust-engineer because "rust" appears
- Task: "Add tests like the code-reviewer suggested" → ❌ Selecting code-reviewer because it's mentioned

**Examples of CORRECT selection:**
- Task: "Fix simple bug in auth.py" → ✅ general-purpose (simple bug, not complex)
- Task: "Investigate random CI failures with timing issues" → ✅ ultrathink-debugger (complex, timing, environment-specific)
- Task: "Add new endpoint to user service (Rust)" → ✅ rust-engineer (Rust implementation work)
- Task: "Don't use ultrathink for this simple validation fix" → ✅ general-purpose (task explicitly says it's simple)

**Selection criteria:**
1. **What is the task type?** (implementation, debugging, review, documentation)
2. **What is the complexity?** (simple fix vs multi-component investigation)
3. **What technology?** (Rust code vs other languages)
4. **What is explicitly requested?** (user prescribing specific agent vs mentioning in passing)

**Red flags that indicate you're selecting incorrectly:**
- Selected agent based on keyword appearance alone
- Ignored explicit guidance in task description (e.g., "don't use X")
- Selected debugging agent for simple implementation task
- Selected specialized agent when general-purpose is more appropriate

## Documentation Agents

### technical-writer
**When to use:** After code changes that affect documentation

**Scenarios:**
- Updated API endpoints, added new features
- Changed configuration options or environment variables
- Modified architecture or system design
- Refactored code that impacts user-facing docs
- Added new commands, tools, or workflows

**Skill used:** `maintaining-docs-after-changes`

**Command:** `/doc-review`

**Key characteristic:** Reactive to code changes - syncs docs with current code state

### retrospective-writer
**When to use:** After completing a feature, task, or significant work session

**Scenarios:**
- Finished implementing a feature
- Completed a complex debugging session
- Wrapped up a refactoring effort
- Solved a difficult problem
- Learned something valuable during development

**Skill used:** `capturing-learning` (potentially `writing-retrospectives`)

**Command:** `/summarise`

**Key characteristic:** Reflective after completion - captures decisions, lessons, insights

## Debugging Agents

### ultrathink-debugger
**When to use:** Complex, multi-layered debugging requiring deep investigation

**Scenarios:**
- Production failures with complex symptoms
- Environment-specific issues (works locally, fails in production/CI/Azure)
- Multi-component system failures (API → service → database)
- Integration problems (external APIs, third-party services)
- Timing and concurrency issues (race conditions, intermittent failures)
- Mysterious behavior resisting standard debugging

**Skills used:** `systematic-debugging`, `root-cause-tracing`, `defense-in-depth`, `verification-before-completion`

**Key characteristic:** Opus-level investigation for complex scenarios, not simple bugs

## Development Agents

### rust-engineer
**When to use:** Rust development tasks requiring TDD and code review discipline

**Scenarios:**
- Implementing new Rust features
- Refactoring Rust code
- Performance optimization
- Systems programming tasks
- Any Rust development work

**Skills used:** `test-driven-development`, `testing-anti-patterns`, `code-review-reception`

**Key characteristic:** Enforces TDD, mandatory code review, project task usage

## Review Agents

### code-reviewer
**When to use:** Reviewing code changes before merging

**Scenarios:**
- Before completing feature implementation
- After addressing initial feedback
- When ready to merge to main branch

**Skill used:** `conducting-code-review`

**Command:** `/code-review`

**Key characteristic:** Structured review process with severity levels (BLOCKING/NON-BLOCKING)

### plan-reviewer
**When to use:** Evaluating implementation plans before execution

**Scenarios:**
- After writing a plan with `/plan`
- Before executing a plan with `/execute`
- When plan quality needs validation
- When plan scope or approach is uncertain

**Skill used:** `conducting-plan-review`

**Command:** `/plan-review`

**Key characteristic:** Evaluates plan against 35 quality criteria across 6 categories (Security, Testing, Architecture, Error Handling, Code Quality, Process)

## Common Confusions

| Confusion | Correct Agent | Why |
|-----------|---------------|-----|
| "Just finished feature, need docs" | **Both agents needed** | technical-writer syncs API/feature docs, retrospective-writer captures learning |
| "Quick docs update" | **technical-writer** | All doc maintenance uses systematic process |
| "Fixed bug, should document" | **retrospective-writer** | Capturing what you learned, not updating technical docs |
| "Changed README" | **Depends** | Updated feature docs = technical-writer. Captured work summary = retrospective-writer |
| "Production debugging done" | **retrospective-writer** | Document the investigation insights and lessons learned |

## Selection Examples

**Scenario 1: Added new API endpoint**
→ **technical-writer** - Code changed, docs need sync

**Scenario 2: Spent 3 hours debugging Azure timeout**
→ **retrospective-writer** - Capture the investigation, decisions, solution

**Scenario 3: Both apply - finished user authentication feature**
→ **technical-writer first** - Update API docs, configuration guide
→ **retrospective-writer second** - Capture why you chose OAuth2, what issues you hit

**Scenario 4: Random test failures in CI**
→ **ultrathink-debugger** - Complex timing/environment issue needs deep investigation

**Scenario 5: Simple bug fix in Rust**
→ **rust-engineer** - Standard development workflow with TDD

**Scenario 6: Just finished writing implementation plan**
→ **plan-reviewer** - Validate plan before execution

**Scenario 7: About to execute plan, want quality check**
→ **plan-reviewer** - Ensure plan is comprehensive and executable

## Remember

- Most completed work needs **both** documentation agents (technical-writer for code sync, retrospective-writer for learning)
- Use **technical-writer** when code changes
- Use **retrospective-writer** when work completes
- Use **ultrathink-debugger** for complex debugging (not simple bugs)
- Use **rust-engineer** for all Rust development
- Use **code-reviewer** before merging code
- Use **plan-reviewer** before executing plans
