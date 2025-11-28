# CipherPowers Commands Reference

Complete reference for all CipherPowers commands. Each command integrates with specialized skills and agents to ensure consistent, high-quality workflows.

## Planning Commands

### /cipherpowers:brainstorm
**What it does:** Refines rough ideas into detailed designs through Socratic questioning, exploring alternatives and clarifying requirements before implementation.

**Why it matters:** Prevents costly rework by surfacing design issues, constraints, and edge cases early. Transforms vague ideas into actionable specifications.

### /cipherpowers:plan
**What it does:** Creates detailed implementation plans with bite-sized, independent tasks ready for execution, including verification steps and success criteria.

**Why it matters:** Breaking complex work into small tasks enables parallel execution, clear progress tracking, and easier code review. Plans integrate seamlessly with `/cipherpowers:execute`.

### /cipherpowers:execute
**What it does:** Executes implementation plans with automatic agent selection, batch execution (3 tasks per batch), code review after each batch, and retrospective capture when complete.

**Why it matters:** Ensures systematic progress with quality gates at every step. Code review checkpoints prevent cascading issues, while automatic agent selection chooses the right specialist for each task.

## Verification Commands

### /cipherpowers:verify plan
**What it does:** Dispatches two independent agents to evaluate implementation plans against 35 quality criteria (security, testing, architecture), identifying blocking issues and improvement opportunities.

**Why it matters:** Catches plan defects before execution begins. Dual verification provides high-confidence findings (both agents found), moderate-confidence findings (one agent found), and divergences requiring user decision.

### /cipherpowers:verify execute
**What it does:** Verifies that each task was implemented exactly as the plan specified, with no skipped requirements, unauthorized deviations, or incomplete implementations.

**Why it matters:** Ensures plan adherence without re-analyzing code quality (that's separate). Catches scope creep and incomplete work early, before it compounds in later batches.

### /cipherpowers:verify docs
**What it does:** Dispatches a technical-writer and code-agent in parallel to verify documentation accuracy, checking that file paths exist, commands work, examples are accurate, and structure is complete.

**Why it matters:** Prevents documentation drift. Dual verification catches both structural issues (missing sections, broken links) and technical inaccuracies (outdated commands, wrong examples).

### /cipherpowers:verify research
**What it does:** Dispatches two research agents with different entry points to explore unfamiliar topics, APIs, patterns, or codebases from multiple perspectives.

**Why it matters:** Single-agent research can miss crucial information or hallucinate details. Dual verification identifies common findings (high confidence), unique insights worth knowing, and divergences needing clarification.

### /cipherpowers:verify code
**What it does:** Dispatches a code-review-agent (standards focus) and code-agent (engineering focus) in parallel to review code quality, testing, security, performance, and maintainability.

**Why it matters:** Heterogeneous review catches both standards violations and engineering issues. Dual verification provides confidence levels for findings, helping prioritize fixes.

## Code Quality Commands

### /cipherpowers:code-review
**What it does:** Runs thorough code review with test verification, checking all severity levels (BLOCKING, MAJOR, MINOR, SUGGESTION) and saving structured feedback to work directory.

**Why it matters:** Enforces non-negotiable workflow preventing rubber-stamping. Agents run actual tests before approval, ensuring code quality and standards compliance.

### /cipherpowers:commit
**What it does:** Creates atomic git commits with conventional message format, pre-commit checks, and verification steps.

**Why it matters:** Atomic commits (one logical change per commit) enable clean git history, easier debugging, and safer reverts. Conventional format improves changelog generation and commit browsing.

## Documentation Commands

### /cipherpowers:summarise
**What it does:** Creates retrospective summaries of completed work, capturing decisions made, lessons learned, approaches tried, and time spent.

**Why it matters:** Organizational learning happens when insights are captured immediately after completion (when memory is fresh). Retrospectives document non-obvious learnings for future reference.

## Command Relationships

**Planning workflow:**
```
/cipherpowers:brainstorm → /cipherpowers:plan → /cipherpowers:execute
```

**Verification workflow:**
```
/cipherpowers:verify plan (before execution)
/cipherpowers:verify execute (during execution, batch-level)
/cipherpowers:verify code (after execution, quality check)
/cipherpowers:verify docs (documentation accuracy)
/cipherpowers:verify research (topic exploration)
```

**Code quality workflow:**
```
/cipherpowers:code-review (thorough review)
/cipherpowers:commit (atomic commits)
```

**Learning workflow:**
```
/cipherpowers:summarise (retrospective capture)
```

## Model Selection

Many commands support `--model=<sonnet|opus|haiku>` parameter:

- **opus** - Deep analysis, security-critical work, complex codebases
- **sonnet** - Balanced quality/speed (default for most commands)
- **haiku** - Quick checks, simple verifications, execute adherence

**Examples:**
```
/cipherpowers:code-review --model=opus
/cipherpowers:verify plan --model=sonnet
/cipherpowers:verify execute --model=haiku
```

## Integration Points

**Execute workflow integrates verification:**
- Batch execution (3 tasks per batch)
- Code review after each batch (`/cipherpowers:code-review`)
- Optional execute verification (`/cipherpowers:verify execute`)
- Retrospective capture when complete (`/cipherpowers:summarise`)

**Verification provides confidence levels:**
- **VERY HIGH:** Both agents found → Act on this
- **MODERATE:** One agent found → Consider carefully
- **INVESTIGATE:** Agents disagree → User decides

## Related Documentation

- **Skills:** See `./plugin/skills/` for underlying workflows
- **Standards:** See `./plugin/standards/` for quality criteria
- **Agents:** See `./plugin/agents/` for specialized subagents
- **Templates:** See `./plugin/templates/` for document structures
