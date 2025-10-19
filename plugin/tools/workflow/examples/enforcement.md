# Enforcement Mode Example

This workflow demonstrates enforcement mode (default). Only STOP conditionals work, ensuring sequential execution.

# Step 1: Check preconditions

Fail: STOP (preconditions failed)

```bash
echo "Checking preconditions..."
```

# Step 2: Run operation

```bash
echo "Running operation..."
```

**Prompt:** Did the operation complete successfully?

# Step 3: Verify results

Fail: STOP (verification failed)

```bash
echo "Verifying results..."
```

# Step 4: Complete

```bash
echo "✓ Workflow complete!"
```
