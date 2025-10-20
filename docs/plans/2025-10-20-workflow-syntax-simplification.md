# Workflow Syntax Simplification - Implementation Plan

**Date:** 2025-10-20
**Status:** Ready for execution
**Approach:** Parallel code and documentation work

## Overview

Simplify workflow syntax by removing "Step" keyword, using clean numbered headings, and enforcing atomic conditional units. This addresses cognitive overhead from the current syntax while making invalid states unrepresentable through stricter type system design.

### Key Refinements (from brainstorming session)

**Type System:**
- `Conditionals` → `Conditions` (more concise, plural form)
- Intermediate `Condition` enum (singular) for parsing - eliminates nested ifs
- Flattened `Action` enum (no named fields in variants)
- `StepNumber` newtype using `NonZeroUsize`

**Parsing Improvements:**
- `strip_separator()` helper - reusable for both headers and conditionals
- Permissive separator parsing - accepts `:`, `-`, `.`, space
- Two-pass validation - strict GOTO target checking
- No regex dependency - clean string operations

**Syntax Simplifications:**
- `STOP message` instead of `STOP (message)` - consistent with `GOTO N` pattern
- Flexible PASS/FAIL separators - `PASS: CONTINUE` or `PASS CONTINUE` both valid
- Clean numbered headings - `## 1. Title` instead of `# Step 1: Title`

## Context

### Current Pain Points

From [2025-10-20 Workflow Syntax Migration](../learning/2025-10-20-workflow-syntax-migration.md):
- 20 minutes lost diagnosing "globally sequential" numbering requirement
- "Step N:" syntax adds noise without value
- Parser can represent invalid states (partial conditionals, missing branches)
- Multiple algorithms per file creates numbering complexity

### Design Decisions (from brainstorming session)

**Refined syntax principles:**
1. **Markdown with strict rules** - Familiar format, clear constraints
2. **Implicit semantics** - Code block = command, no code block = prompt
3. **Atomic conditionals** - Override defaults completely or not at all
4. **ALLCAPS keywords** - PASS, FAIL, GOTO, CONTINUE, STOP
5. **One sequence per file** - No nested workflows, no multiple algorithms

**Type system goal:** Make invalid states unrepresentable

## Complete Type System

After refactoring, the core types will be:

```rust
use std::num::NonZeroUsize;

/// Step number (1-indexed, never zero)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StepNumber(NonZeroUsize);

impl StepNumber {
    pub fn new(n: usize) -> Option<Self> {
        NonZeroUsize::new(n).map(StepNumber)
    }

    pub fn get(&self) -> usize {
        self.0.get()
    }
}

/// Action to take after a step completes
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Continue,               // Proceed to next step
    Stop(Option<String>),   // Halt with optional message
    Goto(StepNumber),       // Jump to specific step
}

/// Atomic conditional unit - both branches always present
#[derive(Debug, Clone, PartialEq)]
pub struct Conditions {
    pub pass: Action,  // Action when command succeeds (exit code 0)
    pub fail: Action,  // Action when command fails (exit code != 0)
}

/// Command to execute
#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub code: String,
    pub quiet: bool,
}

/// A single workflow step
#[derive(Debug, Clone, PartialEq)]
pub struct Step {
    pub number: StepNumber,
    pub title: String,
    pub command: Option<Command>,
    pub conditions: Option<Conditions>,  // None = use implicit defaults
    pub prompt_text: Vec<String>,        // Text content (for prompts)
}

/// Complete workflow
#[derive(Debug, Clone, PartialEq)]
pub struct Workflow {
    pub title: String,
    pub steps: Vec<Step>,
}
```

**Key invariants enforced by types:**
- `StepNumber` cannot be zero (uses `NonZeroUsize`)
- `Conditions` always has both `pass` and `fail` branches
- `Option<Conditions>` distinguishes explicit from implicit defaults
- `Action::Goto` uses `StepNumber`, not raw `usize`

## Complete Example: Before & After

This example demonstrates all syntax changes in a realistic workflow.

### Before (Old Syntax)

```markdown
# Git Commit Readiness Check

# Step 1: Check for uncommitted changes

Fail: STOP (no changes to commit)

```bash
git status --porcelain
```

# Step 2: Run test suite

Pass: Continue
Fail: STOP (tests must pass before commit)

```bash
cargo test
```

# Step 3: Run pre-commit checks

**Prompt:** Review the changes and ensure they are atomic.

Are these changes focused on a single logical change?

Pass: Continue
Fail: Go to Step 6

# Step 4: Check documentation updated

```bash
docs-check
```

# Step 5: Confirm ready to commit

**Prompt:** All checks passed. Ready to commit?

# Step 6: Break changes into smaller commits

**Prompt:** Changes are not atomic. Break them into separate commits first.

Fail: STOP (split changes first)
```

### After (New Syntax)

```markdown
# Git Commit Readiness Check

## 1. Check for uncommitted changes

```bash
git status --porcelain
```

- PASS: CONTINUE
- FAIL: STOP no changes to commit

## 2. Run test suite

```bash
cargo test
```

- PASS: CONTINUE
- FAIL: STOP tests must pass before commit

## 3. Review atomicity

Review the changes and ensure they are atomic.

Are these changes focused on a single logical change?

- PASS: CONTINUE
- FAIL: GOTO 6

## 4. Check documentation updated

```bash
docs-check
```

## 5. Confirm ready to commit

All checks passed. Ready to commit?

## 6. Break changes into smaller commits

Changes are not atomic. Break them into separate commits first.

- FAIL: STOP split changes first
```

### Key Differences Demonstrated

1. **Headers:** `# Step N:` → `## N.` (clean numbered headings)
2. **Keywords:** `Pass:`/`Fail:` → `PASS:`/`FAIL:` (ALLCAPS)
3. **GOTO syntax:** `Go to Step 6` → `GOTO 6` (concise)
4. **STOP syntax:** `STOP (message)` → `STOP message` (no parens)
5. **Implicit prompts:** No `**Prompt:**` prefix needed (text without code block = prompt)
6. **List-based conditionals:** `- PASS: ACTION` format (not paragraph style)
7. **Implicit defaults:** Steps 4 and 5 have no explicit conditionals (use defaults)
8. **Atomic conditionals:** Step 6 shows FAIL explicitly (would need PASS too in real workflow)

## Migration Checklist

When migrating existing workflows to new syntax:

**1. Update headers:**
- ✅ Change `# Step N: Title` → `## N. Title`
- ✅ Ensure all steps use H2 (`##`), not H1 (`#`)
- ✅ Verify sequential numbering (1, 2, 3...) - no gaps or restarts

**2. Update keywords:**
- ✅ Change `Pass:` → `PASS:`
- ✅ Change `Fail:` → `FAIL:`
- ✅ Change `Continue` → `CONTINUE`
- ✅ Change `Go to Step N` → `GOTO N`
- ✅ Change `STOP (message)` → `STOP message`

**3. Update conditional format:**
- ✅ Convert paragraph conditionals to list syntax
- ✅ Use `- PASS: ACTION` format
- ✅ Ensure atomic conditionals (both PASS and FAIL if list present)
- ✅ Remove redundant defaults (e.g., `PASS: CONTINUE` for commands)

**4. Update prompts:**
- ✅ Remove `**Prompt:**` prefix (implicit now)
- ✅ Just write prompt text directly

**5. Validate:**
- ✅ Run `workflow --validate migrated-file.md`
- ✅ Fix any parsing errors
- ✅ Test with `workflow --dry-run migrated-file.md`
- ✅ Verify behavior matches original

## Error Message Examples

The new parser provides clear, actionable error messages:

### Sequential Numbering Error

**Input:**
```markdown
# My Workflow

## 1. First step
...

## 3. Third step  ← Missing step 2
...
```

**Error:**
```
Error: Steps must be numbered sequentially. Expected step 2, found step 3.
Workflows must have exactly one algorithm with continuous numbering (1, 2, 3...).
```

### Atomic Conditional Violation

**Input:**
```markdown
## 1. Check something

```bash
check-command
```

- FAIL: STOP failed  ← Missing PASS branch
```

**Error:**
```
Error: Conditional list must have exactly 2 items (PASS and FAIL). Found 1 item.
Either use implicit defaults (no list) or provide both branches.
```

### Invalid GOTO Target

**Input:**
```markdown
## 1. First step

- PASS: GOTO 10  ← Step 10 doesn't exist
- FAIL: STOP
```

**Error:**
```
Error: Step 1: GOTO target Step 10 does not exist (workflow has 3 steps)
```

### Lowercase Keyword Error

**Input:**
```markdown
## 1. Check

- pass: CONTINUE  ← Should be PASS
- fail: STOP      ← Should be FAIL
```

**Error:**
```
Error: Conditional item must start with PASS or FAIL
Found: pass: CONTINUE
```

### Step Keyword Rejected

**Input:**
```markdown
## Step 1: My step  ← "Step" keyword not allowed
```

**Error:**
```
Error: Failed to parse step header: "Step 1: My step"
Headers must use format: ## N. Title (where N is the step number)
Remove "Step" keyword from headers.
```

## Syntax Specification

### Old Syntax (current)

```markdown
# Step 1: Check for changes

Fail: STOP no changes

```bash
check-has-changes
```

# Step 2: Verify test fails

Pass: Go to Step 6
Fail: Go to Step 2
```

### New Syntax (target)

```markdown
# Git Commit Readiness

## 1. Check for changes

```bash
check-has-changes
```

- PASS: CONTINUE
- FAIL: STOP no changes

## 2. Verify test fails

- PASS: GOTO 6
- FAIL: GOTO 2
```

### Syntax Rules

**Structure:**
- `# Title` = Workflow title (required H1, exactly one)
- `## N Separator Title` = Step (H2, sequential numbering, flexible separator)
- Code block → Command step (implicit: PASS→CONTINUE, FAIL→STOP)
- No code block → Prompt step (implicit: CONTINUE)
- List with PASS/FAIL → Explicit conditionals (atomic unit, both required)

**Keywords (ALLCAPS):**
- `PASS` / `FAIL` - Condition branches (flexible separator: `:`, `-`, `.`, or space)
- `GOTO N` - Jump to step N
- `STOP message` - Halt execution with optional message
- `CONTINUE` - Proceed to next step

**Number parsing (flexible):**
- `## 1. Title` ✅
- `## 1: Title` ✅
- `## 1 — Title` ✅
- `## 1) Title` ✅
- `## 1 - Title` ✅

**NOT supported:**
- `## Step 1: Title` ❌

### Atomic Conditional Principle

**Either:**
- No list → Use implicit defaults
- List present → Must have exactly 2 items (PASS and FAIL)

**Invalid:**
```markdown
- FAIL: STOP  ❌ Missing PASS branch
```

**Valid:**
```markdown
- PASS: CONTINUE
- FAIL: STOP fix first
```

**Also valid (flexible separators):**
```markdown
- PASS CONTINUE
- FAIL - STOP fix first
```

## Implementation Tasks

### Track 1: Type System Refactor (Code)

**Batch 1: Core Type Refinement**

#### Task 1.1: Flatten Action enum

**Current:**
```rust
pub enum Action {
    Continue,
    Stop { message: Option<String> },
    GoToStep { number: usize },
}
```

**Refined:**
```rust
pub enum Action {
    Continue,
    Stop(Option<String>),  // Flatten - no named field
    Goto(StepNumber),      // Flatten + rename
}
```

**File:** `plugin/tools/workflow/src/models.rs`

**Changes:**
- Rename `GoToStep` → `Goto`
- Remove named fields from variants
- Use `StepNumber` newtype instead of raw `usize`

**Test updates:** Update pattern matching in `models.rs` tests

---

#### Task 1.2: Introduce StepNumber newtype

**Add to models.rs:**
```rust
use std::num::NonZeroUsize;

/// Step number (1-indexed, never zero)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StepNumber(NonZeroUsize);

impl StepNumber {
    pub fn new(n: usize) -> Option<Self> {
        NonZeroUsize::new(n).map(StepNumber)
    }

    pub fn get(&self) -> usize {
        self.0.get()
    }
}
```

**File:** `plugin/tools/workflow/src/models.rs`

**Benefits:**
- Cannot represent Step 0 (type system enforces 1-indexed)
- Clear domain type vs generic `usize`
- Copy trait (cheap to pass around)

---

#### Task 1.3: Make Conditions an atomic type

**Current:**
```rust
pub struct Step {
    pub conditionals: Vec<Conditional>,  // Can have 0, 1, or 2 items
}

pub enum Conditional {
    Pass { action: Action },
    Fail { action: Action },
}
```

**Refined:**
```rust
pub struct Step {
    pub conditions: Option<Conditions>,  // Either None or Complete
}

/// Atomic conditional unit - both branches always present
#[derive(Debug, Clone, PartialEq)]
pub struct Conditions {
    pub pass: Action,
    pub fail: Action,
}
```

**File:** `plugin/tools/workflow/src/models.rs`

**Benefits:**
- Cannot represent partial conditionals (one branch missing)
- Type system enforces atomic override principle
- Clear: `None` = defaults, `Some` = explicit logic
- `Conditions` (plural) vs parsing helper `Condition` (singular) - clear distinction

---

**Batch 2: Parser Updates**

#### Task 2.1: Update header parsing for new syntax

**Current:** Parses `# Step N: Title`

**New:** Parse `## N Separator Title`

**Changes to `parser.rs`:**

```rust
fn extract_step_header(text: &str, level: HeadingLevel) -> Option<(StepNumber, String)> {
    // Only accept H2 (##)
    if level != HeadingLevel::H2 {
        return None;
    }

    let trimmed = text.trim();

    // Find where number ends
    let num_end = trimmed.find(|c: char| !c.is_numeric())?;

    // Parse number
    let number = trimmed[..num_end].parse().ok()?;
    let step_number = StepNumber::new(number)?;

    // Strip separator and extract title
    // (See strip_separator helper in Task 2.2 - reusable for conditionals too)
    let title = trimmed[num_end..]
        .trim_start_matches(|c: char| matches!(c, '.' | ':' | '—' | '-' | ')' | ' '))
        .trim()
        .to_string();

    Some((step_number, title))
}
```

**Note:** Separator stripping logic is extracted to reusable `strip_separator()` helper in Task 2.2.

**File:** `plugin/tools/workflow/src/parser.rs`

**Tests to add:**
- `test_parse_heading_with_dot_separator` (## 1. Title)
- `test_parse_heading_with_colon_separator` (## 1: Title)
- `test_parse_heading_with_dash_separator` (## 1 — Title)
- `test_parse_heading_with_paren_separator` (## 1) Title)
- `test_parse_heading_with_just_space` (## 1 Title)
- `test_reject_h1_as_step` (# 1. Title should fail)
- `test_reject_step_keyword` (## Step 1: Title should fail with clear error - see "Step Keyword Rejected" in Error Message Examples)

---

#### Task 2.2: Update conditional parsing for ALLCAPS keywords

**Current:** Parses `Pass:` / `Fail:` / `Go to Step N` / `Continue`

**New:** Parse `PASS` / `FAIL` / `GOTO N` / `CONTINUE` / `STOP message`

**Key improvements:**
- Permissive separator parsing (`:`, `-`, `.`, space)
- Simplified STOP syntax (no parens needed)
- Flat structure using intermediate `Condition` enum
- Reusable `strip_separator()` helper

**Changes to `parser.rs`:**

```rust
/// Strip common separators and whitespace (reusable helper)
fn strip_separator(text: &str) -> &str {
    text.trim_start_matches(|c: char| matches!(c, '.' | ':' | '—' | '-' | ')' | ' '))
        .trim()
}

/// Parsing intermediate type (module-private)
#[derive(Debug, PartialEq)]
enum Condition {
    Pass,
    Fail,
}

fn parse_conditional_item(item: &str) -> Result<(Condition, Action)> {
    let trimmed = item.trim();

    if let Some(rest) = trimmed.strip_prefix("PASS") {
        let action_str = strip_separator(rest);
        Ok((Condition::Pass, parse_action(action_str)?))
    } else if let Some(rest) = trimmed.strip_prefix("FAIL") {
        let action_str = strip_separator(rest);
        Ok((Condition::Fail, parse_action(action_str)?))
    } else {
        anyhow::bail!("Conditional item must start with PASS or FAIL")
    }
}

fn parse_conditional_list(items: &[String]) -> Result<Conditions> {
    let [item1, item2] = items.as_slice() else {
        anyhow::bail!(
            "Conditional list must have exactly 2 items (PASS and FAIL). Found {} items.\n\
             Either use implicit defaults (no list) or provide both branches.",
            items.len()
        );
    };

    let (cond1, action1) = parse_conditional_item(item1)?;
    let (cond2, action2) = parse_conditional_item(item2)?;

    match (cond1, cond2) {
        (Condition::Pass, Condition::Fail) => Ok(Conditions { pass: action1, fail: action2 }),
        (Condition::Fail, Condition::Pass) => Ok(Conditions { pass: action2, fail: action1 }),
        (Condition::Pass, Condition::Pass) => anyhow::bail!("Duplicate PASS branch"),
        (Condition::Fail, Condition::Fail) => anyhow::bail!("Duplicate FAIL branch"),
    }
}

fn parse_action(text: &str) -> Result<Action> {
    let trimmed = text.trim();

    match trimmed {
        "CONTINUE" => Ok(Action::Continue),
        s if s.starts_with("STOP") => parse_stop(s),
        s if s.starts_with("GOTO ") => parse_goto(s),
        _ => anyhow::bail!(
            "Invalid action: {}. Expected CONTINUE, STOP, STOP message, or GOTO N",
            trimmed
        )
    }
}

fn parse_stop(text: &str) -> Result<Action> {
    let message = text.strip_prefix("STOP")
        .unwrap()  // Safe - we already checked starts_with("STOP")
        .trim();

    if message.is_empty() {
        Ok(Action::Stop(None))
    } else {
        Ok(Action::Stop(Some(message.to_string())))
    }
}

fn parse_goto(text: &str) -> Result<Action> {
    let num_str = text.strip_prefix("GOTO ")
        .ok_or_else(|| anyhow::anyhow!("Expected GOTO prefix"))?;

    let number = num_str.parse::<usize>()
        .map_err(|_| anyhow::anyhow!("Invalid GOTO target: {}", num_str))?;

    let step_number = StepNumber::new(number)
        .ok_or_else(|| anyhow::anyhow!("GOTO target must be >= 1, found: {}", number))?;

    Ok(Action::Goto(step_number))
}
```

**File:** `plugin/tools/workflow/src/parser.rs`

**Tests to add:**
- `test_parse_allcaps_keywords`
- `test_reject_lowercase_keywords` (pass/fail should fail - see "Lowercase Keyword Error" in Error Message Examples)
- `test_permissive_separator_parsing` (PASS: vs PASS vs PASS -)
- `test_reject_single_conditional` (only FAIL present - see "Atomic Conditional Violation" in Error Message Examples)
- `test_atomic_conditional_unit` (both PASS and FAIL required)
- `test_duplicate_pass_detection`
- `test_duplicate_fail_detection`
- `test_parse_goto_keyword` (GOTO not "Go to Step")
- `test_parse_stop_with_message` (simplified syntax)
- `test_parse_stop_no_message`

**Note:** Error messages should match examples in "Error Message Examples" section for consistency.

---

#### Task 2.3: Implement list-based conditional detection

**Current:** Parses conditionals from paragraph text

**New:** Parse conditionals from markdown list items

**Parser changes:**

Need to track list items during parsing:

```rust
pub fn parse_workflow(markdown: &str) -> Result<Vec<Step>> {
    let parser = Parser::new(markdown);
    let mut steps = Vec::new();
    let mut current_step: Option<Step> = None;
    let mut in_list = false;
    let mut list_items = Vec::new();
    let mut current_list_item = String::new();

    // ... existing code ...

    for event in parser {
        match event {
            Event::Start(Tag::List(_)) => {
                in_list = true;
                list_items.clear();
            }
            Event::End(Tag::List(_)) => {
                in_list = false;
                if !list_items.is_empty() {
                    // Parse conditional list
                    if let Some(step) = current_step.as_mut() {
                        step.conditions = Some(parse_conditional_list(&list_items)?);
                    }
                    list_items.clear();
                }
            }
            Event::Start(Tag::Item) => {
                current_list_item.clear();
            }
            Event::End(Tag::Item) => {
                if in_list && !current_list_item.is_empty() {
                    list_items.push(current_list_item.clone());
                }
                current_list_item.clear();
            }
            Event::Text(text) => {
                if in_list {
                    current_list_item.push_str(&text);
                } else if /* ... existing step header detection ... */ {
                    // ...
                }
            }
            // ... rest of existing parsing ...
        }
    }

    // ... validation ...
}
```

**File:** `plugin/tools/workflow/src/parser.rs`

**Tests to add:**
- `test_parse_conditional_list`
- `test_detect_list_vs_paragraph_conditionals`
- `test_nested_lists_not_supported` (should error or ignore)

---

#### Task 2.4: Update implicit prompt detection

**Remove explicit `**Prompt:**` requirement**

Steps without code blocks are treated as prompts. The `Step.prompt_text` field collects all paragraph text.

**Parser changes:**

During parsing, collect paragraph text into `prompt_text` field:

```rust
// In parse_workflow loop:
Event::Text(text) => {
    if in_list {
        current_list_item.push_str(&text);
    } else if let Some(step) = current_step.as_mut() {
        // Collect as prompt text if not in code block
        if !in_code_block {
            step.prompt_text.push(text.to_string());
        }
    }
}
```

**Executor logic:**

Steps are categorized at execution time:
- Has `command`? → Execute command, use exit code for conditions
- No `command` but has `prompt_text`? → Display prompt, always CONTINUE (unless explicit conditions)
- No `command`, no `prompt_text`, but has `conditions`? → Control flow only

**Files:**
- `plugin/tools/workflow/src/parser.rs` (collect prompt text)
- `plugin/tools/workflow/src/executor.rs` (step categorization)

**Tests to add:**
- `test_implicit_prompt_no_code_block`
- `test_command_with_code_block`
- `test_control_flow_only_step` (conditionals, no content)

---

**Batch 3: Strict Validation**

#### Task 3.1: Multi-pass validation (sequential numbering + GOTO targets)

**Current:** Single-pass with post-parse validation

**New:** Three-pass strict validation (structure → numbering → GOTO targets)

**Implementation:**

```rust
pub fn parse_workflow(markdown: &str) -> Result<Workflow> {
    // Pass 1: Parse structure, collect step numbers
    let steps = parse_steps(markdown)?;
    let step_count = steps.len();

    // Pass 2: Validate sequential numbering
    for (idx, step) in steps.iter().enumerate() {
        let expected = idx + 1;
        if step.number.get() != expected {
            anyhow::bail!(
                "Steps must be numbered sequentially. Expected step {}, found step {}.\n\
                 Workflows must have exactly one algorithm with continuous numbering (1, 2, 3...).",
                expected,
                step.number.get()
            );
        }
    }

    // Pass 3: Validate GOTO targets
    for step in &steps {
        if let Some(conditions) = &step.conditions {
            validate_goto_target(&conditions.pass, step_count, step.number)?;
            validate_goto_target(&conditions.fail, step_count, step.number)?;
        }
    }

    Ok(Workflow { steps })
}

fn validate_goto_target(action: &Action, step_count: usize, current_step: StepNumber) -> Result<()> {
    if let Action::Goto(target) = action {
        if target.get() > step_count {
            anyhow::bail!(
                "Step {}: GOTO target Step {} does not exist (workflow has {} steps)",
                current_step.get(),
                target.get(),
                step_count
            );
        }
    }
    Ok(())
}
```

**Note:** Pass 3 validates both `conditions.pass` and `conditions.fail` branches for every step that has explicit conditions.

**File:** `plugin/tools/workflow/src/parser.rs`

**Tests to add:**
- `test_sequential_numbering_valid` (1, 2, 3... should succeed)
- `test_sequential_numbering_gap` (1, 2, 4 should error - see "Sequential Numbering Error" in Error Message Examples)
- `test_sequential_numbering_restart` (1, 2, 1 should error)
- `test_sequential_numbering_wrong_start` (0, 1, 2 should error - caught by StepNumber)
- `test_goto_validation_after_numbering` (three-pass order)
- `test_goto_target_beyond_workflow` (should error - see "Invalid GOTO Target" in Error Message Examples)
- `test_goto_valid_target` (should succeed)

**Note:** Ensure error messages match examples in "Error Message Examples" section.

---

#### Task 3.2: Add --validate flag

**Add CLI flag:**

```rust
// In main.rs
#[derive(Parser)]
struct Cli {
    // ... existing fields ...

    /// Validate workflow structure without executing
    #[arg(long)]
    validate: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.validate {
        validate_workflow_file(&cli.workflow_file)?;
        println!("✓ Workflow is valid");
        return Ok(());
    }

    // ... existing execution logic ...
}

fn validate_workflow_file(path: &Path) -> Result<()> {
    let markdown = fs::read_to_string(path)?;
    let _workflow = parse_workflow(&markdown)?;

    // Additional validation rules
    // ...

    Ok(())
}
```

**File:** `plugin/tools/workflow/src/main.rs`

**Validation checks:**
- ✅ Exactly one H1 (workflow title)
- ✅ All steps are H2
- ✅ Sequential numbering (1, 2, 3...)
- ✅ GOTO targets exist (two-pass validation)
- ✅ Conditional lists have exactly 2 items (PASS and FAIL)
- ✅ No duplicate PASS or FAIL branches
- ✅ Keywords are ALLCAPS
- ✅ No "Step" keyword in headers

**Tests to add:**
- `test_validate_flag_success`
- `test_validate_flag_catches_errors`
- Integration tests for each validation rule

---

**Batch 4: Executor Updates**

#### Task 4.1: Update executor for new Conditions type

**Current:** Iterates over `Vec<Conditional>`

**New:** Handles `Option<Conditions>`

```rust
fn evaluate_step_outcome(step: &Step, exit_code: i32) -> Action {
    match &step.conditions {
        Some(conditions) => {
            if exit_code == 0 {
                conditions.pass.clone()
            } else {
                conditions.fail.clone()
            }
        }
        None => {
            // Implicit defaults
            if step.command.is_some() {
                // Command step: PASS → CONTINUE, FAIL → STOP
                if exit_code == 0 {
                    Action::Continue
                } else {
                    Action::Stop(None)
                }
            } else {
                // Prompt step: always CONTINUE
                Action::Continue
            }
        }
    }
}
```

**File:** `plugin/tools/workflow/src/executor.rs`

**Tests to update:**
- All executor tests using old Conditional enum
- Add tests for implicit defaults vs explicit conditions
- Update all references from `conditionals` → `conditions`

---

#### Task 4.2: Update GOTO execution logic

**Current:** `GoToStep { number: usize }`

**New:** `Goto(StepNumber)`

```rust
fn execute_action(action: Action, steps: &[Step], current_index: usize) -> Result<usize> {
    match action {
        Action::Continue => Ok(current_index + 1),
        Action::Stop(reason) => {
            if let Some(msg) = reason {
                anyhow::bail!("Workflow stopped: {}", msg);
            } else {
                anyhow::bail!("Workflow stopped");
            }
        }
        Action::Goto(target) => {
            // Convert 1-indexed StepNumber to 0-indexed array index
            Ok(target.get() - 1)
        }
    }
}
```

**File:** `plugin/tools/workflow/src/executor.rs`

**Tests to update:**
- All goto-related executor tests
- Update pattern matching for flattened Action enum

---

#### Task 4.3: Add --dry-run flag for validation without execution

**Purpose:** Parse and validate workflows without executing commands (still displays prompts)

**CLI changes:**

```rust
// In main.rs
#[derive(Parser)]
struct Cli {
    // ... existing fields ...

    /// Validate workflow without executing commands (prompts still displayed)
    #[arg(long)]
    dry_run: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.validate {
        validate_workflow_file(&cli.workflow_file)?;
        println!("✓ Workflow is valid");
        return Ok(());
    }

    // Parse workflow
    let markdown = fs::read_to_string(&cli.workflow_file)?;
    let workflow = parse_workflow(&markdown)?;

    // Execute with dry-run mode if requested
    execute_workflow(workflow, cli.dry_run)?;

    Ok(())
}
```

**Executor changes:**

```rust
pub fn execute_workflow(workflow: Workflow, dry_run: bool) -> Result<()> {
    let mut current_index = 0;

    while current_index < workflow.steps.len() {
        let step = &workflow.steps[current_index];

        println!("\n## Step {}: {}", step.number.get(), step.title);

        let exit_code = if let Some(command) = &step.command {
            if dry_run {
                // Dry-run mode: show command but don't execute
                println!("```bash");
                println!("{}", command.code);
                println!("```");
                println!("[DRY-RUN] Command not executed");
                0  // Assume success in dry-run mode
            } else {
                // Normal mode: execute command
                execute_command(&command.code, command.quiet)?
            }
        } else if !step.prompt_text.is_empty() {
            // Display prompt (in both dry-run and normal mode)
            for line in &step.prompt_text {
                println!("{}", line);
            }
            // Wait for user confirmation
            pause_for_user()?;
            0  // Prompts always "succeed"
        } else {
            // Control flow only
            0
        };

        // Evaluate conditions and determine next step
        let action = evaluate_step_outcome(step, exit_code);
        current_index = execute_action(action, &workflow.steps, current_index)?;
    }

    Ok(())
}

fn pause_for_user() -> Result<()> {
    use std::io::{self, Write};
    print!("\nPress Enter to continue...");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}
```

**File:** `plugin/tools/workflow/src/main.rs`
**File:** `plugin/tools/workflow/src/executor.rs`

**Behavior:**
- `--dry-run`: Show commands (don't execute), display prompts (with pause), follow conditionals assuming success
- Regular mode: Execute commands, display prompts (with pause), follow conditionals based on actual exit codes
- `--validate`: Parse only, no execution, no prompts

**Tests to add:**
- `test_dry_run_skips_commands`
- `test_dry_run_shows_prompts`
- `test_dry_run_follows_success_path` (assumes exit code 0)
- `test_normal_mode_executes_commands`
- `test_validate_vs_dry_run_vs_normal`

---

### Track 2: Documentation Updates

**Batch 5: Syntax Documentation**

#### Task 5.1: Update workflow README with complete example

**File:** `plugin/tools/workflow/README.md`

**Changes:**
- Add complete before/after example from plan (see "Complete Example: Before & After" section above)
- Replace all syntax examples with new format
- Document atomic conditional principle
- Add ALLCAPS keyword reference
- Document flexible number parsing
- Add --validate and --dry-run flag documentation
- Remove old "Step N:" syntax examples

**Structure:**
```markdown
# Workflow Tool

## Complete Example

[Include before/after example from plan]

## Syntax

### Structure
- `# Title` - Workflow title (required H1)
- `## N. Step Name` - Steps (H2, sequential)
- Code blocks - Commands
- No code block - Prompts
- List syntax - Explicit conditionals

### Keywords (ALLCAPS)
- PASS / FAIL
- GOTO N
- STOP message (optional)
- CONTINUE

### Atomic Conditionals
Either use defaults or override both branches...

## Usage Modes

### Execution (default)
`workflow file.md` - Execute commands, display prompts, follow conditionals

### Dry-run mode
`workflow --dry-run file.md` - Show commands (don't execute), display prompts, assume success

### Validation only
`workflow --validate file.md` - Parse and validate structure, no execution

## Migration Guide

[Include migration checklist from plan]

## Additional Examples

[Full examples in new syntax]
```

---

#### Task 5.2: Update git-commit-algorithm.md

**File:** `plugin/practices/git-commit-algorithm.md`

**Migration:**
- Convert `# Step N:` → `## N.`
- Convert `Pass: Go to Step N` → `- PASS: GOTO N`
- Convert `Fail: STOP (reason)` → `- FAIL: STOP reason`
- Update implicit defaults (remove redundant conditionals)

**Example transformation:**

Before:
```markdown
# Step 1: Check for changes

Fail: STOP no changes

```bash
check-has-changes
```
```

After:
```markdown
## 1. Check for changes

```bash
check-has-changes
```

- PASS: CONTINUE
- FAIL: STOP no changes
```

---

#### Task 5.3: Update TDD enforcement algorithm

**File:** `plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md`

**Migration:**
- Convert Decision Algorithm (Steps 1-6)
- Convert Recovery Algorithm (Steps 7-11)
- Ensure global sequential numbering maintained
- Update all conditionals to list syntax where explicit

**Note:** This file has two algorithms - ensure numbering continues globally (1-11, not 1-6 then 1-5)

---

#### Task 5.4: Update code review skill

**File:** `plugin/skills/conducting-code-review/SKILL.md`

**Migration:**
- Convert workflow sections to new syntax
- Update examples in documentation
- Maintain decision tree structure

---

#### Task 5.5: Update algorithmic enforcement skill

**File:** `plugin/skills/meta/algorithmic-command-enforcement/SKILL.md`

**Changes:**
- Update all workflow examples to new syntax
- Document atomic conditional principle
- Add new syntax to "how to write algorithms" section

---

**Batch 6: Practice Documentation**

#### Task 6.1: Update workflow practice

**File:** `plugin/practices/workflow.md`

**Changes:**
- Update syntax examples
- Document new keywords
- Add validation guidance
- Remove old syntax references

---

#### Task 6.2: Create syntax migration retrospective

**File:** `docs/learning/2025-10-20-workflow-syntax-simplification.md`

**Content:**
- What was done (parser refactor + syntax migration)
- Design decisions (atomic conditionals, ALLCAPS keywords)
- What went well (type system prevents invalid states)
- What went poorly (migration challenges, test updates)
- Insights discovered (two-pass validation, implicit semantics)
- Metrics (LOC changed, tests updated, files migrated)

---

## Execution Strategy

### Parallel Tracks

**Track 1 (Code):** Batches 1-4 (Type system → Parser → Validation → Executor)
**Track 2 (Docs):** Batches 5-6 (Syntax docs → Practice docs)

**Checkpoints:**
- After Batch 1: Type system compiles, existing tests updated
- After Batch 2: Parser handles new syntax, validation strict
- After Batch 3: --validate flag works
- After Batch 4: Executor works with new types
- After Batch 5: All workflow files migrated
- After Batch 6: Documentation complete

**Code Review:**
- After Batch 2 (parser changes are critical)
- After Batch 4 (before documentation migration)
- After Batch 6 (final review before merge)

### Testing Strategy

**Unit tests:**
- Update all existing tests for new types
- Add new tests for each task (listed inline above)

**Integration tests:**
- Test --validate flag on valid workflows
- Test --validate flag catches all error types
- Test execution with new syntax
- Test migration of existing workflows

**Verification:**
```bash
# After each batch
cargo test
cargo clippy
cargo build --release

# After Batch 3 (validation works)
workflow --validate plugin/practices/git-commit-algorithm.md

# After Batch 4 (dry-run and execution work)
workflow --dry-run plugin/practices/git-commit-algorithm.md
workflow --dry-run plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md

# After Batch 5 (all migrated workflows valid)
workflow --validate plugin/practices/git-commit-algorithm.md
workflow --validate plugin/skills/testing/tdd-enforcement-algorithm/SKILL.md
workflow --validate plugin/skills/conducting-code-review/SKILL.md
workflow --validate plugin/skills/meta/algorithmic-command-enforcement/SKILL.md
```

## Success Criteria

**Code:**
- ✅ All tests passing
- ✅ Clippy warnings resolved
- ✅ Type system prevents invalid states (cannot represent partial conditionals)
- ✅ --validate flag catches all syntax errors
- ✅ Two-pass GOTO validation works
- ✅ Executor handles new types correctly

**Documentation:**
- ✅ All workflow files migrated to new syntax
- ✅ README.md updated with complete example and migration guide
- ✅ Practice files use new syntax in examples
- ✅ Retrospective captures learning

**Integration:**
- ✅ Existing workflows execute correctly
- ✅ Error messages are clear and actionable (see Error Message Examples section)
- ✅ Migration checklist provided in README and plan
- ✅ --dry-run mode works for testing workflows without execution

## Risks and Mitigations

**Risk:** Breaking existing workflows during migration

**Mitigation:**
- Work done in dedicated worktree (`.worktrees/workflow-syntax-refactor`)
- Main branch remains stable during development
- Validate each migrated file with --validate flag before committing
- Test with --dry-run before actual execution
- Easy rollback: abandon worktree if problems arise

**Risk:** Two-pass parsing adds complexity

**Mitigation:**
- Local tool, workflows are small (<50 steps typical)
- Performance impact negligible
- Type safety benefit outweighs complexity cost

**Risk:** Test updates across many files

**Mitigation:**
- Systematic batch approach
- Update tests alongside implementation (not after)
- Use compiler errors as checklist

## Timeline Estimate

**Batch 1 (Type system refactor):** ~2-3 hours
- Type updates, StepNumber newtype, Conditions struct
- Update all existing tests

**Batch 2 (Parser updates):** ~4-5 hours
- Header parsing, conditional parsing, list detection
- Implicit prompt detection
- New tests for each task (15+ tests)

**Batch 3 (Validation):** ~3-4 hours
- Three-pass validation (structure → numbering → GOTO)
- --validate flag implementation
- Integration tests for validation rules

**Batch 4 (Executor):** ~3-4 hours
- Update for new Conditions type
- GOTO execution with StepNumber
- --dry-run flag implementation
- Executor tests update

**Batch 5 (Syntax documentation):** ~3-4 hours
- README with complete example and migration guide
- Migrate 4 workflow files (git-commit-algorithm, TDD, code review, algorithmic enforcement)

**Batch 6 (Practice documentation):** ~2 hours
- Update workflow practice
- Create retrospective

**Implementation subtotal:** ~17-22 hours

**Code reviews:** ~3-4 hours across 3 checkpoints (after Batch 2, 4, 6)

**Debugging/fixes:** ~2-3 hours (unexpected test failures, edge cases)

**Grand total:** ~22-29 hours

**Note:** Includes --dry-run implementation, complete example, migration checklist, and improved error messages not in original estimate.

## Follow-Up Work (Future)

- Consider migration tool: `workflow migrate old.md new.md`
- Improve error messages with line numbers
- Add syntax highlighting for editors (VSCode extension?)
- Consider workflow visualization (`workflow --graph`)

## References

- [Workflow Syntax Migration Retrospective](../learning/2025-10-20-workflow-syntax-migration.md)
- [Algorithmic Command Enforcement Skill](../../plugin/skills/meta/algorithmic-command-enforcement/SKILL.md)
- Current parser: `plugin/tools/workflow/src/parser.rs`
- Current models: `plugin/tools/workflow/src/models.rs`
