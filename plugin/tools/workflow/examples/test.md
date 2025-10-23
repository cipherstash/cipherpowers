# Example Commit Workflow

This example demonstrates workflow syntax features including commands, prompts, conditionals, and control flow.

## 1. Check for uncommitted changes

```bash
git status --short
```

- PASS: CONTINUE
- FAIL: STOP no changes to commit

## 2. Review changes

Review the staged and unstaged changes. Are all changes related to a single logical change?

- PASS: CONTINUE
- FAIL: GOTO 7

## 3. Run tests

```bash
cargo test --lib
```

- PASS: CONTINUE
- FAIL: STOP fix tests first

## 4. Run checks

```bash
cargo clippy --all-targets -- -D warnings
```

- PASS: CONTINUE
- FAIL: STOP fix clippy warnings

## 5. Format code

```bash
cargo fmt
```

## 6. Create commit

```bash
git add -A
git commit  # Use conventional-commits.md format
```

## 7. Split changes

The changes span multiple concerns. Split into separate commits:

1. Stage files for first logical change
2. Create commit for that change
3. Repeat until all changes are committed
