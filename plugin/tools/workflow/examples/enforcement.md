# Enforcement Mode Example

This workflow demonstrates enforcement mode (default). Only STOP conditionals work, ensuring sequential execution.

## 1. Check preconditions

Fail: STOP (preconditions failed)

```bash
echo "Checking preconditions..."
```

## 2. Run operation

```bash
echo "Running operation..."
```

## 3. Run operation with prompt

```bash
echo "Running operation with prompt..."
```

**Prompt:** Did the operation complete successfully?

## 4. Verify results

Fail: STOP (verification failed)

```bash
echo "Verifying results..."
```

## 5. Complete

```bash
echo "✓ Workflow complete!"
```
