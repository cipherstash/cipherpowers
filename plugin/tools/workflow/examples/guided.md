# Guided Mode Example

This workflow demonstrates guided mode. All conditionals work, enabling flexible control flow.

# Step 1: Check if work needed

```bash quiet
git status --porcelain
```

→ If output empty: STOP (nothing to do)
→ Otherwise: Continue

# Step 2: Quick check

```bash
echo "Running quick check..."
```

→ Exit 0: Go to Step 4
→ Exit ≠ 0: Continue

# Step 3: Detailed analysis

```bash
echo "Running detailed analysis (only if quick check failed)..."
```

# Step 4: Confirm action

**Prompt:** Ready to proceed?

# Step 5: Execute

```bash
echo "Executing action..."
```
