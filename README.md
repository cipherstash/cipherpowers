# CipherPowers

Comprehensive development toolkit for Claude Code.

## Discovery Tools

**Find Skills:**
```bash
./plugin/tools/find-skills "pattern"
./plugin/tools/find-skills --local "pattern"    # cipherpowers only
./plugin/tools/find-skills --upstream "pattern" # superpowers only
```

**Find Practices:**
```bash
./plugin/tools/find-practices "pattern"
./plugin/tools/find-practices --local "pattern"    # cipherpowers only
./plugin/tools/find-practices --upstream "pattern" # marketplace only
```

## Direct References

In agents and commands, use:
- `@${CIPHERPOWERS_ROOT}/plugin/docs/practices/practice-name.md` - Direct practice reference
- `@${SUPERPOWERS_SKILLS_ROOT}/skills/category/skill-name/SKILL.md` - Skill reference
- `@plugin/docs/practices/practice-name.md` - Relative to plugin root
- `@skills/category/skill-name/SKILL.md` - Relative to plugin root

## Documentation

See `CLAUDE.md` for complete architecture and usage guide.
