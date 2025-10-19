# Enforcement Mode Example

This workflow demonstrates enforcement mode (default). All conditionals except STOP are ignored, ensuring sequential execution.

# Step 1: Check preconditions

```bash
echo "Checking preconditions..."
```

→ Exit 0: Continue
→ Exit ≠ 0: STOP (preconditions failed)

# Step 2: Run operation

```bash
echo "Running operation..."
```

**Prompt:** Did the operation complete successfully?

# Step 3: Verify results

```bash
echo "Verifying results..."
```

→ Exit 0: Continue
→ Exit ≠ 0: STOP (verification failed)

# Step 4: Complete

```bash
echo "Workflow complete!"
```
