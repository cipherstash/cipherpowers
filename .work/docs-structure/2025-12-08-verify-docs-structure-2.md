# Documentation Structure Review #2

## Skill Requirements Summary

The organizing-documentation skill recommends intent-based structure with these key requirements:

1. **Intent-Based Organization:** BUILD/FIX/UNDERSTAND/LOOKUP (not content type)
2. **BUILD Structure:** Numbered phases (00-START, 01-DESIGN, 02-IMPLEMENT, 03-TEST, 04-VERIFY)
3. **FIX Organization:** By symptoms (not root causes)
4. **LOOKUP Constraint:** All items < 30 seconds to find
5. **INDEX.md:** Master index with purpose annotations (mandatory)
6. **README-Per-Directory:** Every directory needs consistent README structure
7. **Dual Navigation:** NAVIGATION.md (task-based) + INDEX.md (concept-based)
8. **Redirects:** Old locations should have README files pointing to new structure
9. **Naming Conventions:** ALLCAPS for types, numeric prefixes for sequences, lowercase-dashes for content

## Current Structure Analysis

### docs/ Directory
- **Location:** `/Users/tobyhede/src/cipherpowers/docs/`
- **Current contents:**
  - `configuring-project-commands.md` (single file)
- **Structure:** Flat, no BUILD/FIX/UNDERSTAND/LOOKUP organization
- **Navigation:** No INDEX.md, no NAVIGATION.md
- **READMEs:** None

### plugin/docs/ Directory
- **Location:** `/Users/tobyhede/src/cipherpowers/plugin/docs/`
- **Current contents:**
  - `AGENTS.md` (reference document)
  - `COMMANDS.md` (reference document)
  - `SKILLS.md` (reference document)
  - `WORKFLOW.md` (narrative workflow guide)
- **Structure:** All documents at root level, no BUILD/FIX/UNDERSTAND/LOOKUP organization
- **Navigation:** No INDEX.md, no NAVIGATION.md
- **READMEs:** None

### Document Type Analysis

| Document | Purpose Intent | Current Location | Skill Classification |
|----------|---|---|---|
| `WORKFLOW.md` | Guide developers through brainstorm→plan→execute | plugin/docs/WORKFLOW.md | BUILD/00-START (entry point) |
| `COMMANDS.md` | Command reference with descriptions | plugin/docs/COMMANDS.md | LOOKUP (quick reference) |
| `SKILLS.md` | Skills list with descriptions | plugin/docs/SKILLS.md | LOOKUP (quick reference) |
| `AGENTS.md` | Agents reference | plugin/docs/AGENTS.md | LOOKUP (quick reference) |
| `configuring-project-commands.md` | How to configure project commands | docs/configuring-project-commands.md | BUILD/02-IMPLEMENT (patterns/how-to) |

## Alignment Assessment

### Aligned
- **Document quality:** All existing documents are well-written with clear purposes
- **File naming conventions:** Uses ALLCAPS.md for document types (AGENTS.md, COMMANDS.md, etc.)
- **Content organization:** Documents have clear sections and are logically structured
- **Purpose clarity:** Each document has a clear purpose and useful content

### Gaps (CRITICAL)

1. **Missing PRIMARY STRUCTURE:** No BUILD/FIX/UNDERSTAND/LOOKUP directory structure
   - All plugin/docs files are at root level
   - No directory organization by developer intent
   - Developers cannot browse by "what they want to do"

2. **Missing INDEX.md:** No master index with purpose annotations in either docs directory
   - No single place showing all documentation with purposes
   - Violates "INDEX.md is mandatory" requirement
   - No excluded categories list (work/, plans/, archives/)

3. **Missing NAVIGATION.md:** No task-based primary navigation document
   - Skills recommend dual navigation (NAVIGATION.md + INDEX.md)
   - Currently only have scattered documents

4. **Missing README-Per-Directory:** Neither docs/ nor plugin/docs/ has README.md
   - Cannot explain purpose or usage of each directory
   - Violates "every directory MUST have README.md" requirement
   - New developers don't know what each directory contains

5. **Missing FIX Section:** No symptom-based debugging documentation
   - Only fixes for project-specific issues, not cipherpowers issues
   - Could be deliberate (cipherpowers is plugin, not app) but needs explicit decision

6. **Missing UNDERSTAND Section:** No deep learning/architecture documentation
   - No docs explaining cipherpowers architecture
   - No evolution/why documents
   - CLAUDE.md covers this in main file, but not in organized structure

7. **Missing BUILD/00-START:** No mandatory prerequisites directory
   - WORKFLOW.md serves this purpose but not in proper structure
   - Should be in BUILD/00-START/ with clear entry point

8. **Missing dual-navigation:** Instruction files (CLAUDE.md/AGENTS.md) reference plugin/docs/ but don't integrate with skill-recommended structure

### Issues

**CRITICAL Issues (block adoption of skill):**

1. **No intent-based directory structure** (CRITICAL)
   - Skill explicitly requires BUILD/FIX/UNDERSTAND/LOOKUP structure
   - Current flat structure doesn't match developer mental model
   - Users cannot navigate by intent

2. **Missing INDEX.md in both docs directories** (CRITICAL)
   - Skill: "Skip the INDEX.md" is listed as anti-pattern
   - Prevents discovery and overview
   - No excluded categories list

3. **Missing README.md files** (CRITICAL)
   - Every directory must have README with purpose/use-cases
   - Current directories have zero READMEs
   - Violates documented pattern

**HIGH Issues (significant gaps):**

4. **plugin/docs/ lacks directory structure** (HIGH)
   - 4 root-level files (AGENTS.md, COMMANDS.md, SKILLS.md, WORKFLOW.md)
   - No subdirectories for logical grouping
   - Cannot be browsed by intent

5. **docs/ directory underpopulated** (HIGH)
   - Only 1 file (configuring-project-commands.md)
   - Should contain project setup/onboarding docs
   - No BUILD/00-START structure

6. **No redirects from old locations** (HIGH)
   - If reorganized, users with bookmarks may be confused
   - Skill recommends creating README files in old locations

**MEDIUM Issues (quality concerns):**

7. **Missing progressive disclosure** (MEDIUM)
   - No time-budget entry points (5 min / 20 min / 2 hours)
   - WORKFLOW.md does this well, but not systematically across docs
   - Not every document has TL;DR section

8. **Missing role-based reading paths** (MEDIUM)
   - Different users (team leads, plugin users, contributors) have different needs
   - No documented reading sequences per role

9. **Metadata visibility inconsistent** (MEDIUM)
   - Some docs have status/dates, others don't
   - Skill recommends explicit visibility (Last Updated, Status, etc.)
   - Not systematically applied

## Recommendations

### Immediate Actions (Must Complete)

1. **Create plugin/docs/ directory structure** (CRITICAL)
   ```
   plugin/docs/
   ├── README.md                    # Directory purpose
   ├── INDEX.md                      # Master index with purposes
   ├── BUILD/                        # Building with CipherPowers
   │   ├── 00-START/
   │   │   └── README.md
   │   ├── 01-DESIGN/
   │   │   └── README.md
   │   ├── 02-IMPLEMENT/
   │   │   └── README.md
   │   ├── 03-TEST/
   │   │   └── README.md
   │   └── 04-VERIFY/
   │       └── README.md
   ├── FIX/                         # Fixing Issues
   │   ├── README.md
   │   ├── symptoms/
   │   ├── investigation/
   │   └── solutions/
   ├── UNDERSTAND/                  # Learning CipherPowers
   │   ├── README.md
   │   ├── core-systems/
   │   └── evolution/
   └── LOOKUP/                      # Quick References
       └── README.md
   ```

2. **Create docs/ directory structure** (CRITICAL)
   ```
   docs/
   ├── README.md                    # Project documentation purpose
   ├── INDEX.md                      # Master index
   ├── BUILD/                        # Building the plugin
   │   ├── 00-START/
   │   │   ├── README.md
   │   │   └── prerequisites.md
   │   ├── 01-DESIGN/
   │   │   └── README.md
   │   ├── 02-IMPLEMENT/
   │   │   ├── README.md
   │   │   └── architecture.md       # Move from UNDERSTAND
   │   ├── 03-TEST/
   │   │   └── README.md
   │   └── 04-VERIFY/
   │       └── README.md
   └── UNDERSTAND/                  # Understanding the Plugin
       ├── README.md
       ├── core-systems/
       │   ├── README.md
       │   ├── three-layer-architecture.md
       │   ├── skills-layer.md
       │   ├── automation-layer.md
       │   └── documentation-layer.md
       └── evolution/
           ├── README.md
           └── design-decisions.md
   ```

3. **Create INDEX.md files** (CRITICAL)
   - **plugin/docs/INDEX.md:** Master reference index for all plugin documentation
   - **docs/INDEX.md:** Master reference index for all project documentation
   - Both must include:
     - Purpose annotations for every document
     - Last Updated date
     - Total document count
     - Excluded Categories section

4. **Create README.md files** (CRITICAL)
   - Every directory needs README with:
     - Purpose statement (1-2 sentences)
     - "Use this section when:" list
     - Navigation to contents
     - Quick links to related sections
   - Use template: `${CLAUDE_PLUGIN_ROOT}templates/documentation-readme-template.md`

5. **Move/reorganize existing documents** (HIGH)
   - **WORKFLOW.md** → **BUILD/00-START/workflow.md**
   - **COMMANDS.md** → **LOOKUP/commands.md**
   - **SKILLS.md** → **LOOKUP/skills.md**
   - **AGENTS.md** → **LOOKUP/agents.md**
   - **configuring-project-commands.md** → **BUILD/02-IMPLEMENT/configuring-project-commands.md**

### Follow-Up Actions (Should Complete)

6. **Add progressive disclosure** (MEDIUM)
   - Every document should have TL;DR section
   - Provide 5-min, 20-min, 2-hour entry points
   - Include read time estimates

7. **Create role-based reading paths** (MEDIUM)
   - Document reading sequences for different roles:
     - Plugin users (operators)
     - Plugin contributors (developers)
     - Team leads (decision makers)
     - New onboarding (initial orientation)

8. **Create FIX section** (MEDIUM)
   - If applicable: troubleshooting guides organized by symptoms
   - Common plugin setup issues
   - Debugging workflow problems

9. **Add metadata visibility** (LOW)
   - Status indicators (✅ / ⚠️ / ☠️)
   - Last verified dates
   - Version information (e.g., for skills)

10. **Update CLAUDE.md/AGENTS.md references** (LOW)
    - Instruction files reference plugin/docs/ structure
    - After reorganization, update paths to match new structure
    - Use `cipherpowers:maintaining-instruction-files` skill

### Decision Points

**Clarification needed:**
1. Should `docs/` contain only cipherpowers development docs, or also user-facing docs? (Current assumption: development only)
2. Should FIX section include troubleshooting guides for users, or only for developers?
3. Are there role-based reading paths needed for different user types?

## Verification Checklist

- [ ] All docs audited and categorized by intent
- [ ] BUILD/00-START has prerequisites
- [ ] FIX organized by symptoms (if applicable)
- [ ] LOOKUP items are < 30 second lookups
- [ ] INDEX.md created with purpose column
- [ ] README.md in every directory with consistent structure
- [ ] Old locations have redirects (if reorganized)
- [ ] Internal links verified to work
- [ ] CLAUDE.md/AGENTS.md references updated
- [ ] Progressive disclosure added (TL;DR, time budgets)

## Conclusion

**Alignment Level:** 5% aligned with organizing-documentation skill requirements

The current documentation structure is **not organized by developer intent** as required by the skill. Documents exist and have good content, but the lack of BUILD/FIX/UNDERSTAND/LOOKUP structure, missing INDEX.md files, and absent README.md files prevent the skill from being fully adopted.

**Primary blocker:** Absence of intent-based directory structure (BUILD/FIX/UNDERSTAND/LOOKUP). This is the foundational requirement that enables all other patterns.

**Recommendation:** Implement all CRITICAL and HIGH items to achieve 80%+ alignment with skill requirements.
