# CipherPowers Workflow

The recommended workflow for implementing features and tackling complex tasks.

## Overview

CipherPowers provides a structured three-step workflow designed to prevent scope creep, maintain quality, and capture learning:

**Brainstorm → Plan → Execute**

This workflow scales from simple tasks (skip steps as needed) to complex multi-file implementations.

## 1. Brainstorm

**Command:** `/cipherpowers:brainstorm`

### When to use

- You have an idea but need to refine requirements
- Design options need exploration
- Requirements are unclear or incomplete
- Multiple approaches exist and you need to evaluate trade-offs
- Edge cases and constraints need identification

### What it does

The brainstorm command launches an interactive Socratic dialogue to clarify your thinking:

- **Explores constraints:** What are the technical and business limitations?
- **Identifies edge cases:** What could go wrong? What unusual scenarios exist?
- **Evaluates trade-offs:** What are the pros and cons of different approaches?
- **Refines requirements:** Transforms vague ideas into concrete, actionable designs
- **Results in clarity:** Produces a clear understanding ready for planning

**Output:** Design documents saved to `.work/<topic>/` directory

### Skip if

You already have a fully-detailed design spec with:
- Clear requirements
- Identified constraints
- Evaluated alternatives
- Edge cases documented

### Example Session

```
You: "I want to add caching to our API"

Brainstorm agent:
- What data needs caching?
- How long should cache entries live?
- What invalidation strategy?
- Memory constraints?
- Cache hits vs misses monitoring?
- Fallback behavior when cache unavailable?

Result: Clear design doc covering TTL strategy, invalidation triggers, monitoring approach
```

## 2. Plan

**Command:** `/cipherpowers:plan`

### When to use

- After brainstorming (or if you already have a clear design)
- Ready to break down implementation
- Task is non-trivial (multiple files, complex logic)
- You want structured guidance for execution
- You want automatic agent selection during execution

### What it does

Creates a structured implementation plan optimized for the execute workflow:

- **Breaks down work:** Divides implementation into manageable tasks
- **Sizes for batches:** Each task designed for 3-task execution batches
- **Provides instructions:** Step-by-step guidance with expected outcomes
- **Enables automation:** Execute command can auto-select agents per task type
- **Saves plan:** Implementation plan saved to `.work/` directory

**Output:** Plan file in `.work/` directory (e.g., `.work/2025-11-28-add-caching-plan.md`)

### Skip if

Task is trivial:
- Single file modification
- Less than 10 lines of code
- No architectural decisions
- Simple refactoring or typo fixes

### Plan Structure

A good plan includes:

```markdown
## Task 1: [Description]
**Files:** path/to/file.ts
**Instructions:** Detailed step-by-step
**Expected outcome:** What should result

## Task 2: [Description]
...
```

Tasks are grouped in batches of 3 for execution with code review checkpoints between batches.

### Optional: Plan Verification

Before executing, verify plan quality:

```
/cipherpowers:verify plan
```

This uses dual-agent review to evaluate:
- Completeness (are all requirements covered?)
- Clarity (can an engineer execute without guessing?)
- Feasibility (are tasks appropriately sized?)
- Quality gates (are verification steps included?)

## 3. Execute

**Command:** `/cipherpowers:execute [plan-file-path]`

### When to use

- You have a plan file ready (from `/cipherpowers:plan`)
- Ready to implement
- Want automatic agent selection for task types
- Want batch execution with review checkpoints

### What it does

Orchestrates plan execution with quality gates and automatic specialization:

#### Automatic Agent Selection

For each task, the execute command:
1. **Analyzes task type:** Code changes? Documentation? Testing?
2. **Selects specialized agent:** rust-agent, code-agent, technical-writer, etc.
3. **Provides task context:** Agent receives only its specific task
4. **Executes with constraints:** Agent follows non-negotiable workflow

See `./plugin/skills/selecting-agents/SKILL.md` for agent selection logic.

#### Batch Execution

Tasks are processed in batches of 3:

```
Batch 1: Tasks 1-3
  → Execute each task with specialized agent
  → Code review checkpoint
  → Address feedback before proceeding

Batch 2: Tasks 4-6
  → Execute each task
  → Code review checkpoint
  → Address feedback

...continue until complete
```

#### Code Review Checkpoints

After each batch:
- Automatic code review of all batch changes
- Severity-based feedback (BLOCKING vs NON-BLOCKING)
- Must address BLOCKING issues before next batch
- Maintains quality throughout execution

#### Completion

When all tasks complete:
- Optional execute verification (`/cipherpowers:verify execute`) checks implementation vs plan (on-demand, not automatic)
- Prompt for retrospective to capture learning
- Summary of work completed

### Why Batch Execution Matters

**Prevents scope creep:**
- Each batch has clear boundaries
- Review catches deviations early
- Maintains focus on plan requirements

**Catches issues early:**
- Review after 3 tasks vs after 30 tasks
- Smaller change sets = easier review
- Feedback loop is tighter

**Maintains consistency:**
- Patterns established in early batches
- Review ensures consistency across batches
- Large implementations stay coherent

**Captures learning:**
- Retrospective prompt when complete
- Lessons documented for future reference
- Team knowledge grows over time

### Optional: Execute Verification

After execution completes, verify implementation matches plan:

```
/cipherpowers:verify execute
```

This uses dual-agent review to check:
- Plan adherence (were all tasks completed?)
- Requirement coverage (are all features implemented?)
- Quality gates passed (tests passing, standards met?)

**Note:** This verification is on-demand, not automatic. Use when:
- Implementation was complex
- Multiple agents were involved
- You want confirmation before merging
- Plan had many tasks

### Example Execution Flow

```
$ /cipherpowers:execute .work/2025-11-28-add-caching-plan.md

Analyzing plan: 9 tasks found, 3 batches

=== Batch 1 (Tasks 1-3) ===
Task 1: Add cache interface → code-agent
Task 2: Implement Redis cache → rust-agent
Task 3: Add cache configuration → code-agent

Code review checkpoint...
✓ All clear, proceeding to Batch 2

=== Batch 2 (Tasks 4-6) ===
...

=== Completion ===
✓ All 9 tasks complete
✓ All tests passing

Would you like to create a retrospective? [y/n]
```

## Alternative: Direct Work

For simple tasks that don't need a plan:

### 1. Work Directly in Session

Just describe what you want:

```
You: "Add a debug log statement to the authenticate function"

Claude: [makes the change]
```

### 2. Code Review When Ready

```
/cipherpowers:code-review
```

Triggers manual code review of your session changes:
- Identifies issues (BLOCKING, NON-BLOCKING)
- Provides structured feedback
- Saves review to `.work/` directory

### 3. Commit Changes

```
/cipherpowers:commit
```

Creates conventional commit:
- Pre-commit checks (tests, linting if configured)
- Atomic commit guidelines
- Conventional commit message format

## Verification Commands

Use verification at any point in your workflow:

| Command | Purpose | When to Use |
|---------|---------|-------------|
| `/cipherpowers:verify plan` | Evaluate plan quality before execution | After creating plan, before execute |
| `/cipherpowers:verify execute` | Check implementation vs plan adherence | After execute completes (on-demand) |
| `/cipherpowers:verify docs` | Find documentation gaps and issues | After code changes, before merging |
| `/cipherpowers:verify research` | Verify research thoroughness and coverage | After research phase, before decisions |

All verification commands use **dual-agent review** for higher confidence:

### How Dual Verification Works

1. **Two independent agents** analyze the same subject
2. **Findings categorized** by confidence level:
   - **Common issues (VERY HIGH confidence):** Both agents found it
   - **Exclusive issues (MODERATE confidence):** Only one agent found it
   - **Divergences (INVESTIGATE):** Agents disagree
3. **Collation report** presents all findings with confidence levels
4. **Saved to `.work/`** for reference

### Verification vs Execution

**Verification is read-only:**
- Finds issues and gaps
- Produces structured report
- No files modified

**Execution applies fixes:**
- Uses verification report as input
- Makes actual changes
- Updates documentation/code

**Pattern:** Verify → Plan fixes → Execute fixes

## Full Workflow Example

### Scenario: Add Authentication to API

**Step 1: Brainstorm**

```
/cipherpowers:brainstorm

You: "I want to add JWT authentication to our API"

Agent:
- Which endpoints need auth?
- Token lifetime?
- Refresh token strategy?
- Permission levels needed?
- Rate limiting?
- Existing user database or new?

Result: Design doc in .work/jwt-auth/design.md
```

**Step 2: Plan**

```
/cipherpowers:plan

You: "Create implementation plan for JWT auth based on the design doc"

Agent creates plan with tasks:
1. Add JWT dependencies
2. Create auth middleware
3. Add user model
4. Implement token generation
5. Implement token validation
6. Add auth endpoints
7. Protect existing endpoints
8. Add tests
9. Update documentation

Result: .work/2025-11-28-jwt-auth-plan.md
```

**Step 3: Verify Plan (Optional)**

```
/cipherpowers:verify plan

Two agents review plan independently
Collation report identifies:
✓ Plan is complete
⚠️ Consider adding token blacklist for logout
⚠️ Missing rate limiting on auth endpoints

You review findings, update plan if needed
```

**Step 4: Execute**

```
/cipherpowers:execute .work/2025-11-28-jwt-auth-plan.md

Batch 1 (Tasks 1-3): Dependencies, middleware, user model
→ Code review → Address feedback

Batch 2 (Tasks 4-6): Token operations, endpoints
→ Code review → Address feedback

Batch 3 (Tasks 7-9): Protection, tests, docs
→ Code review → Address feedback

All tasks complete!
```

**Step 5: Verify Execute (Optional)**

```
/cipherpowers:verify execute

Two agents verify implementation vs plan
Collation report shows:
✓ All tasks completed
✓ All requirements met
✓ Tests passing
⚠️ One endpoint missing rate limit (non-blocking)

You review findings, decide if fixes needed
```

**Step 6: Verify Documentation**

```
/cipherpowers:verify docs

Two agents check documentation
Collation report shows:
✗ README missing auth setup instructions (blocking)
✗ API docs missing new endpoints (blocking)
⚠️ Examples need auth headers (non-blocking)

You fix documentation issues
```

**Step 7: Commit**

```
/cipherpowers:commit

Pre-commit checks run
Conventional commit created:
"feat(auth): add JWT authentication to API endpoints"

Work complete!
```

**Step 8: Retrospective**

```
/cipherpowers:summarise

Captures learning:
- JWT implementation patterns
- Rate limiting considerations
- Documentation sync process
- Testing strategies for auth

Saved to docs/ for team reference
```

## When to Skip Steps

**Skip brainstorm if:**
- ✓ Requirements are crystal clear
- ✓ Design is fully specified
- ✓ No alternatives to evaluate

**Skip plan if:**
- ✓ Task is trivial (< 10 lines, single file)
- ✓ Simple refactoring or typo fix
- ✗ "I know what I'm doing" (scope creep risk!)

**Skip verification if:**
- ✓ Task is very simple
- ✓ Changes are low-risk
- ✗ "We're in a hurry" (bugs cost more later!)

## Tips for Success

**Planning:**
- Break tasks into small, testable chunks
- Include expected outcomes for each task
- Size tasks for 3-task batches
- Add verification steps to plan

**Execution:**
- Trust the batch process (don't skip review)
- Address BLOCKING feedback immediately
- Let execute command select agents
- Use verify execute for complex implementations

**Verification:**
- Verify early and often
- Don't skip documentation verification
- Review exclusive issues carefully
- Investigate divergences before proceeding

**Learning:**
- Always create retrospectives for significant work
- Document decisions and trade-offs
- Share learning with team
- Update practices based on lessons

## Next Steps

**Try the workflow:**
1. Pick a real feature to implement
2. Start with `/cipherpowers:brainstorm`
3. Follow through to completion
4. Note what works and what doesn't

**Customize for your team:**
- Adjust verification timing
- Configure quality gates
- Add project-specific agents
- Document your workflow variants

**Learn more:**
- See `CLAUDE.md` for architecture details
- Browse `./plugin/skills/` for workflow guides
- Check `./plugin/standards/` for practices
- Read `./plugin/examples/` for templates
