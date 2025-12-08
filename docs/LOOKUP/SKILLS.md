# CipherPowers Skills Reference

Skills are automatically discovered by Claude Code. Invoke with the Skill tool: `Skill(skill: "cipherpowers:skill-name")`

For detailed information about each skill, use the Skill tool or read the skill's SKILL.md file in `./plugin/skills/`.

## Development Skills

### test-driven-development
Use when implementing any feature or bugfix, before writing implementation code - write the test first, watch it fail, write minimal code to pass; ensures tests actually verify behavior by requiring failure first

### tdd-enforcement-algorithm
Algorithmic decision tree enforcing test-first development via boolean conditions instead of imperatives

### systematic-debugging
Use when encountering any bug, test failure, or unexpected behavior, before proposing fixes - four-phase framework (root cause investigation, pattern analysis, hypothesis testing, implementation) that ensures understanding before attempting solutions

### root-cause-tracing
Use when errors occur deep in execution and you need to trace back to find the original trigger - systematically traces bugs backward through call stack, adding instrumentation when needed, to identify source of invalid data or incorrect behavior

### defense-in-depth
Use when invalid data causes failures deep in execution, requiring validation at multiple system layers - validates at every layer data passes through to make bugs structurally impossible

## Planning & Execution Skills

### brainstorming
Use when creating or developing, before writing code or implementation plans - refines rough ideas into fully-formed designs through collaborative questioning, alternative exploration, and incremental validation. Don't use during clear 'mechanical' processes

### writing-plans
Use when design is complete and you need detailed implementation tasks for engineers with zero codebase context - creates comprehensive implementation plans with exact file paths, complete code examples, and verification steps assuming engineer has minimal domain knowledge

### executing-plans
Use when partner provides a complete implementation plan to execute in controlled batches with review checkpoints - loads plan, reviews critically, executes tasks in batches, reports for review between batches

### following-plans
Algorithmic decision tree for when to follow plan exactly vs when to report BLOCKED - prevents scope creep and unauthorized deviations

### verifying-plans
Complete workflow for evaluating implementation plans before execution with quality checklist and structured feedback

## Code Review Skills

### conducting-code-review
Complete workflow for conducting thorough code reviews with test verification and structured feedback

### requesting-code-review
Use when completing tasks, implementing major features, or before merging to verify work meets requirements - dispatches cipherpowers:code-review-agent subagent to review implementation against plan or requirements before proceeding

### receiving-code-review
Use when receiving code review feedback, before implementing suggestions, especially if feedback seems unclear or technically questionable - requires technical rigor and verification, not performative agreement or blind implementation

### validating-review-feedback
Validate code review feedback against implementation plan to prevent scope creep and derailment

## Documentation Skills

### organizing-documentation
Set up or reorganize project documentation using intent-based structure (BUILD/FIX/UNDERSTAND/LOOKUP)

### maintaining-docs-after-changes
Two-phase workflow to keep project documentation synchronized with code changes

### creating-research-packages
Document complex domain knowledge as self-contained packages with multiple reading paths

### documenting-debugging-workflows
Create symptom-based debugging documentation that matches how developers actually search for solutions

### creating-quality-gates
Establish workflow boundary checklists with clear pass/fail criteria and escalation procedures

### capturing-learning
Systematic retrospective to capture decisions, lessons, and insights from completed work

## Workflow Skills

### commit-workflow
Systematic commit process with pre-commit checks, atomic commits, and conventional commit messages

### dual-verification
Use two independent agents for reviews or research, then collate findings to identify common findings, unique insights, and divergences

### finishing-a-development-branch
Use when implementation is complete, all tests pass, and you need to decide how to integrate the work - guides completion of development work by presenting structured options for merge, PR, or cleanup

### using-git-worktrees
Use when starting feature work that needs isolation from current workspace or before executing implementation plans - creates isolated git worktrees with smart directory selection and safety verification

### subagent-driven-development
Use when executing implementation plans with independent tasks in the current session - dispatches fresh subagent for each task with code review between tasks, enabling fast iteration with quality gates

### dispatching-parallel-agents
Use when facing 3+ independent failures that can be investigated without shared state or dependencies - dispatches multiple Claude agents to investigate and fix independent problems concurrently

## Meta Skills

### using-cipherpowers
Use when starting any conversation - establishes mandatory workflows for finding and using skills, including using Skill tool before announcing usage, following brainstorming before coding, and creating TodoWrite todos for checklists

### selecting-agents
Decision guide for choosing the right specialized agent for each task type

### writing-skills
Use when creating new skills, editing existing skills, or verifying skills work before deployment - applies TDD to process documentation by testing with subagents before writing, iterating until bulletproof against rationalization

### testing-skills-with-subagents
Use when creating or editing skills, before deployment, to verify they work under pressure and resist rationalization - applies RED-GREEN-REFACTOR cycle to process documentation by running baseline without skill, writing to address failures, iterating to close loopholes

### algorithmic-command-enforcement
Use boolean decision trees instead of imperatives for 100% compliance under pressure

### sharing-skills
Use when you've developed a broadly useful skill and want to contribute it upstream via pull request - guides process of branching, committing, pushing, and creating PR to contribute skills back to upstream repository

## Testing Skills

### testing-anti-patterns
Use when writing or changing tests, adding mocks, or tempted to add test-only methods to production code - prevents testing mock behavior, production pollution with test-only methods, and mocking without understanding dependencies

### systematic-type-migration
Safe refactoring workflow for replacing old types with new type-safe implementations through integration-test-first, file-by-file migration with incremental verification

---

## Usage Examples

**Invoke a skill:**
```
Skill(skill: "cipherpowers:test-driven-development")
```

**Read a skill file directly:**
```
@${CLAUDE_PLUGIN_ROOT}skills/conducting-code-review/SKILL.md
```

**Browse skills by category:**
Skills are organized in `./plugin/skills/` by name. Each skill directory contains:
- `SKILL.md` - The skill documentation
- Optional supporting files (examples, test scenarios, etc.)

## Related Documentation

- **Standards:** See `./plugin/standards/` for project-specific coding standards and practices
- **Templates:** See `./plugin/templates/` for templates (agent, practice, skill)
- **Commands:** See `./plugin/commands/` for slash commands that dispatch to skills
- **Agents:** See `./plugin/agents/` for specialized agents that reference skills
