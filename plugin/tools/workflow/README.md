# Workflow Executor

Executes markdown-based workflows with shell commands, conditionals, and prompts.

## Security Warning

**⚠️ IMPORTANT:** Workflow files execute arbitrary shell commands with the same permissions as the user running the workflow tool.

**Only run workflow files you trust.** Malicious workflow files could:
- Delete files
- Exfiltrate data
- Execute arbitrary code
- Modify system state

Treat workflow files like shell scripts - review them before execution.

**Best practices:**
- Version control workflow files in your repository
- Require code review before merging workflow changes
- Do not accept workflows from untrusted sources
- Review workflow markdown before executing

## Installation

```bash
cargo install --path .
```

## Usage

```bash
workflow path/to/workflow.md
```

See `docs/plans/2025-10-19-workflow-executor.md` for implementation details.
