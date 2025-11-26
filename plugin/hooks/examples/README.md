# Example Gate Configurations

## strict.json

Block on all quality gate failures. Use when:
- Working on production code
- Quality standards are non-negotiable
- Team is experienced with tooling

Copy to use:
```bash
cp ${CLAUDE_PLUGIN_ROOT}/hooks/examples/strict.json .claude/gates.json
```

## permissive.json

Warn on failures but continue. Use when:
- Prototyping or experimenting
- Learning new tooling
- Quality checks are aspirational

Copy to use:
```bash
cp ${CLAUDE_PLUGIN_ROOT}/hooks/examples/permissive.json .claude/gates.json
```

## pipeline.json

Chain gates in sequence (format → check → test → build). Use when:
- Want auto-formatting before checks
- Need ordered quality verification
- Building complex workflows

Copy to use:
```bash
cp ${CLAUDE_PLUGIN_ROOT}/hooks/examples/pipeline.json .claude/gates.json
```

## Customization

Mix and match concepts:
- Use strict mode for some gates, permissive for others
- Chain critical gates, run others independently
- Enable different gates for different agents/tools
