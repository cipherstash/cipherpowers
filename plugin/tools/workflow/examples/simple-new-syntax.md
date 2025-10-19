# Simple Workflow Example (New Syntax)

This demonstrates the simplified Pass/Fail syntax.

**Note:** This file shows syntax patterns for documentation purposes. It is NOT an executable workflow (steps restart numbering in each section). For executable examples, see `enforcement.md` and `guided.md`.

## Minimal (Most Common)

# Step 1: Run tests

```bash
mise run test
```

# Step 2: Build

```bash
mise run build
```

# Step 3: Deploy

```bash
mise run deploy
```

Any failure stops automatically (implicit Fail → STOP).

## With Custom Messages

# Step 1: Run tests

Fail: STOP (fix tests before deploying)

```bash
mise run test
```

# Step 2: Build

Fail: STOP (build failed - check logs)

```bash
mise run build
```

## With Control Flow

# Step 1: Check if already deployed

Pass: Go to Step 3  # Already deployed, skip build
Fail: Continue      # Not deployed, need to build

```bash quiet
mise run check-deployment
```

# Step 2: Build and deploy

```bash
mise run build && mise run deploy
```

# Step 3: Verify deployment

```bash
mise run verify
```
