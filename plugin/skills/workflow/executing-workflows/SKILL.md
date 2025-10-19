---
name: Executing Workflows
description: Use workflow executor tool to run markdown-based workflows with enforcement or guided modes
when_to_use: when task has existing workflow file, when algorithmic enforcement needed, when following multi-step process
applies_to: all agents
related_skills: creating-workflows
version: 1.0.0
---

# Executing Workflows

## Overview

Execute markdown-based workflows using the workflow executor tool. Use enforcement mode for algorithmic compliance (100% vs 33% imperative) or guided mode for flexible process guidance.

**Announce at start:** "I'm using the workflow executor to run [workflow-name]."

**Core principle:** Always use the workflow tool when a workflow file exists. Don't rationalize "I can follow it manually" - that's how compliance drops from 100% to 33%.

## When to Use This Skill

**Use workflow executor when:**
- Workflow markdown file exists for the task
- Algorithmic enforcement required (git commit, TDD, code review triggers)
- Following documented multi-step process
- Need verifiable compliance with process

**Don't use when:**
- No workflow file exists (consider if one should - see creating-workflows skill)
- Single-step task (no process to enforce)
- Pure research/exploration (no defined steps)

## Related Skills

- **Creating workflows:** `@${CLAUDE_PLUGIN_ROOT}skills/workflow/creating-workflows/SKILL.md`
- **Workflow practice:** `@${CLAUDE_PLUGIN_ROOT}practices/workflow.md`
