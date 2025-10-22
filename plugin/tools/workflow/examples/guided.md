# Guided Mode Example

Run with: `workflow --guided examples/guided.md`

This workflow demonstrates guided mode. All conditionals work (Pass, Fail, GoTo), enabling flexible control flow.

## 1. Check prerequisites

Fail: STOP (prerequisites not met)

```bash
echo "Checking prerequisites..."
exit 0
```

If prerequisites check fails, stop early.

## 2. Quick check

Pass: Go to Step 4
Fail: Continue

```bash
echo "Running quick check..."
exit 0
```

If quick check passes, skip detailed analysis (Go to Step 4).

## 3. Detailed analysis

Only runs if quick check failed (Step 2 Fail: Continue).

```bash
echo "Running detailed analysis (only if quick check failed)..."
```

## 4. Confirm action

**Prompt:** Ready to proceed?

## 5. Execute

```bash
echo "Executing action..."
```

## 6. Complete

```bash
echo "✓ Workflow complete!"
```
