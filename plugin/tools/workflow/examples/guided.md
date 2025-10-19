# Guided Mode Example

This workflow demonstrates guided mode. All conditionals work, enabling flexible control flow.

# Step 1: Check if work needed

**Prompt:** Are there uncommitted changes? If no, STOP (nothing to do)

```bash quiet
git status --porcelain
```

# Step 2: Quick check

```bash
echo "Running quick check..."
```

Pass: Go to Step 4

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
