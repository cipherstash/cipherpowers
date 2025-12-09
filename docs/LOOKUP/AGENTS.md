# CipherPowers Agents Reference

## TL;DR

**Agents are specialized subagents automatically dispatched by commands** - code-agent and rust-agent handle development, code-review-agent and plan-review-agent handle review, technical-writer handles documentation, and research-agent handles research verification.

---

Specialized subagents that handle specific tasks with enforced workflows.

## Development Agents

### code-agent
**Purpose:** Meticulous principal software engineer who implements features with TDD and mandatory code review.
**When used:** Automatically selected for general development tasks (non-Rust languages).

### rust-agent
**Purpose:** Principal Rust engineer specializing in modern Rust patterns and systems programming.
**When used:** Automatically selected for Rust development tasks.

### ultrathink-debugger
**Purpose:** Complex debugging specialist for production issues requiring deep investigation across system boundaries.
**When used:** Manually invoked for multi-component failures, environment-specific issues, timing problems, or mysterious behavior.

## Review Agents

### code-review-agent
**Purpose:** Principal engineer who conducts thorough code reviews with test verification against standards.
**When used:** Dispatched by `/cipherpowers:code-review` or automatically during batch checkpoints in `/cipherpowers:execute`.

### plan-review-agent
**Purpose:** Evaluates implementation plans for completeness, structure, and quality before execution.
**When used:** Dispatched by `/cipherpowers:verify plan` for dual-verification of implementation plans.

### execute-review-agent
**Purpose:** Verifies completed batch implementation matches plan specification exactly (plan adherence, not code quality).
**When used:** Dispatched by `/cipherpowers:verify execute` to check if tasks were completed as specified.

### review-collation-agent
**Purpose:** Compares two independent reviews to identify common issues, exclusive issues, and divergences with confidence levels.
**When used:** Automatically dispatched after dual-verification reviews (plan, code, execute, docs, research) to synthesize findings.

## Support Agents

### commit-agent
**Purpose:** Systematic git committer ensuring atomic commits with conventional commit messages.
**When used:** Dispatched by `/cipherpowers:commit` to create well-formed, atomic commits.

### research-agent
**Purpose:** Thorough researcher who explores topics from multiple angles with evidence gathering and gap identification.
**When used:** Dispatched by `/cipherpowers:verify research` for dual-verification research tasks.

### technical-writer
**Purpose:** Documentation maintenance specialist with mode detection (verification finds issues, execution applies fixes).
**When used:** Dispatched by `/cipherpowers:verify docs` (verification mode) or `/cipherpowers:execute` (execution mode) for documentation tasks.

### gatekeeper
**Purpose:** Validates code review feedback against implementation plan to prevent scope creep and derailment.
**When used:** Automatically dispatched during `/cipherpowers:execute` between code review and fixing to validate review feedback.

## Agent Selection

Commands automatically select appropriate agents based on task type. See the `selecting-agents` skill for manual selection guidance.
