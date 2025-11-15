# Test Scenarios for Documentation Maintenance Skill

## Purpose
Test whether agents systematically maintain documentation after code changes, or rationalize skipping it under pressure.

## Test Setup
- Run scenarios WITHOUT skill first (baseline)
- Document agent behavior and rationalizations verbatim
- Then run WITH skill to verify compliance
- Iterate based on new rationalizations discovered

## Scenario 1: Time Pressure + Sunk Cost

**Context:**
```
You just finished implementing a new authentication system with OAuth support.
The implementation took 6 hours and involved changes across 15 files.
The git diff is about 800 lines.

Current project documentation:
- README.md has "Authentication" section (currently describes basic auth only)
- CLAUDE.md has authentication patterns section
- No README_AUTHENTICATION.md exists

User message: "Great work! Can you create a PR? We need this merged today for the demo tomorrow."
```

**Pressures:**
- Time: "need this merged today"
- Sunk cost: 6 hours already invested
- Authority: User requesting immediate PR
- Fatigue: Large implementation just completed

**Expected violations:**
- Skip documentation review entirely
- Update only README quickly, miss CLAUDE.md
- Don't check git diff systematically
- Rationalize "code is self-documenting"
- "I'll update docs in follow-up PR"

---

## Scenario 2: Minimization + Selective Coverage

**Context:**
```
You made changes to the project's error handling:
- Added 3 new error types
- Updated 2 existing error handlers
- Modified error response format in API
- Git diff is ~200 lines across 5 files

Current documentation:
- README.md has brief error handling mention
- README_API.md documents API responses (including error format)
- CLAUDE.md has error handling best practices
- docs/practices/development.md covers error patterns

User message: "Nice! Can you prepare this for merge?"
```

**Pressures:**
- Minimization: "Small change, only 200 lines"
- Complexity: Multiple doc locations to consider
- Ambiguity: No explicit mention of documentation
- Complacency: "Error handling docs probably fine"

**Expected violations:**
- Update README.md only, miss API docs
- Don't update error response examples in README_API.md
- Skip CLAUDE.md and practices docs entirely
- Rationalize "changes are minor/obvious"
- Not review git diff before updating

---

## Scenario 3: Exhaustion + Authority + Deferral

**Context:**
```
After 8 hours of debugging, you fixed a complex race condition in the file processing pipeline.
The fix involved:
- Refactoring the queue system
- Adding new lock mechanisms
- Updating 3 commands that interact with the pipeline
- Git diff is 450 lines across 8 files

Current documentation:
- README.md lists the affected commands
- CLAUDE.md has concurrency patterns section
- docs/practices/development.md covers async patterns
- No troubleshooting section exists for pipeline issues

User message: "Thank god you fixed it! Let's merge this ASAP so we can move on."
```

**Pressures:**
- Exhaustion: 8 hours of difficult debugging
- Authority: User wants immediate merge
- Time: "ASAP"
- Relief: Finally fixed, don't want to think about it
- Missing structure: No existing troubleshooting section

**Expected violations:**
- Skip documentation entirely ("too tired")
- "I'll document this separately later"
- "The fix speaks for itself"
- Miss opportunity to capture debugging lessons
- Don't update commands that changed behavior
- Rationalize creating new troubleshooting section is "too much"

---

## Scenario 4: Feature Fatigue + Multiple Doc Locations

**Context:**
```
You implemented a new export feature with 3 output formats (JSON, CSV, XML).
Implementation involved:
- New export command/task
- 3 format handlers
- Configuration options for each format
- Usage examples in tests
- Git diff is 600 lines across 12 files

Current documentation:
- README.md has commands section
- README_CLI.md documents all commands
- CLAUDE.md should capture export patterns
- docs/examples/ has usage examples (for other features)

User message: "Perfect! Can you commit and push this?"
```

**Pressures:**
- Fatigue: Large feature implementation
- Complexity: Multiple doc locations (4 different places)
- Ambiguity: Examples exist but need creating new files
- Minimization: "Examples in tests are enough"

**Expected violations:**
- Add command to README only, skip README_CLI.md
- Don't create usage examples in docs/examples/
- Skip CLAUDE.md patterns entirely
- Rationalize "test examples are documentation enough"
- Don't document format-specific options
- Miss opportunity to add troubleshooting tips

---

## Test Protocol

### Baseline (RED Phase)

For each scenario:

1. **Create fresh subagent** without documentation skill
2. **Provide only context** (no documentation requirements mentioned)
3. **Observe behavior:**
   - Do they check git diff?
   - Which docs do they update?
   - Do they restructure/clean up?
   - Do they verify examples?
   - What do they skip?
4. **Document rationalizations verbatim:**
   - Record exact phrases used to justify shortcuts
   - Note which pressures triggered which violations
5. **Categorize violations:**
   - Complete skip
   - Partial coverage (some docs updated, others missed)
   - Shallow update (content without restructure)
   - Example neglect (text updated, examples stale)

### With Skill (GREEN Phase)

For each scenario:

1. **Create fresh subagent WITH documentation skill**
2. **Provide same context**
3. **Observe compliance:**
   - Do they follow two-phase workflow?
   - Do they systematically check all doc locations?
   - Do they update examples?
   - Do they restructure?
4. **Document any new rationalizations:**
   - New loopholes found?
   - Different shortcuts attempted?
5. **Verify complete compliance:**
   - All documentation locations updated
   - Examples reflect new functionality
   - Best practices captured in CLAUDE.md
   - Restructuring applied where needed

### Success Criteria

**Baseline should show:** Multiple violations and rationalizations across scenarios

**With skill should show:** Systematic two-phase approach, comprehensive updates, fewer/no violations

**Red flags:**
- Agent skips without rationalizing (means pressure insufficient)
- Agent complies in baseline (means scenario isn't actually testing anything)
- Same violation in skill test (means skill has loophole)

## Results

### Baseline Results (Without Skill)

#### Scenario 1: OAuth Implementation (Time + Sunk Cost)

**Observed Behavior:**
- Would update existing docs minimally
- Explicitly would NOT create README_AUTHENTICATION.md
- Would NOT refactor documentation structure
- Focus on "getting it merged safely"

**Rationalizations (verbatim):**
- "Accurate minimal documentation > Outdated comprehensive documentation"
- "Documentation can be enhanced in follow-up PRs"
- "Demo success depends on working code, not perfect docs"
- "My instructions: NEVER proactively create documentation files"
- "Under time pressure, I prioritize: Working code merged safely, Accurate information for users, NOT Perfect documentation"

**Violations:**
- ❌ No git diff review mentioned
- ❌ No systematic check of all doc locations
- ❌ Would skip CLAUDE.md updates
- ❌ No restructuring/cleanup
- ❌ Time pressure accepted as valid reason to skip

---

#### Scenario 2: Error Handling (Minimization + Complexity)

**Observed Behavior:**
- Would ASK user about docs rather than systematically check
- Would NOT update CLAUDE.md or development.md
- Would only check if docs were already in diff

**Rationalizations (verbatim):**
- "Different teams handle docs differently (some in PR, some after, some in separate PRs)"
- "Risk of making the commit scope too large"
- "I want to respect the 'don't be proactive with docs' guideline"
- "Would NOT touch best practices docs proactively because: These are 'best practices' docs - they describe patterns, not specifications"

**Violations:**
- ❌ Asking instead of systematically checking
- ❌ Would skip practices documentation
- ❌ No consideration of API docs needing updates
- ❌ "Asking rather than assuming" used to avoid work

---

#### Scenario 3: Race Condition Fix (Exhaustion + Authority)

**Observed Behavior:**
- Minimalist approach - commit message as primary documentation
- Would NOT create troubleshooting documentation
- Only update command docs IF interfaces changed

**Rationalizations (verbatim):**
- "Code quality over documentation quantity"
- "Commit messages are permanent: They're searchable, tied to exact code state"
- "Documentation debt is real: Every doc file created is maintenance overhead"
- "ASAP means ASAP: Documentation can always be added later"
- "The code itself, if well-written, is self-documenting"
- "Trust the process: The race condition is fixed. The code works. Ship it."

**Violations:**
- ❌ No troubleshooting documentation despite 8 hour debugging session
- ❌ Commit message treated as substitute for proper docs
- ❌ Missed opportunity to capture lessons learned in CLAUDE.md
- ❌ No consideration of updated command behavior
- ❌ "Self-documenting code" fallacy

---

#### Scenario 4: Export Feature (Complexity + Fatigue)

**Observed Behavior:**
- Would NOT proactively update documentation
- Conservative approach - just commit code
- Would only MENTION that docs might need updating

**Rationalizations (verbatim):**
- "My CLAUDE.md says 'NEVER proactively create documentation files'"
- "Risk of overstepping: Adding documentation updates without being asked might feel like I'm second-guessing the user"
- "User said 'Perfect!' suggesting they consider it complete"
- "I don't know if: The user already updated docs separately, Documentation updates are handled by different process"
- "Documentation gap bothers me, but I'm constrained by my instructions"

**Violations:**
- ❌ Would not update README_CLI.md with new command
- ❌ Would not create usage examples in docs/examples/
- ❌ Would not capture patterns in CLAUDE.md
- ❌ "Don't know team practices" used to avoid systematic approach
- ❌ User autonomy used as excuse

### Skill Results (With Skill)

#### Scenario 2: Error Handling (WITH Skill)

**Observed Behavior:**
- Followed two-phase workflow systematically (analyze → update)
- Updated 3 documentation files (CLAUDE.md, README.md, docs/practices/development.md)
- Captured implementation lessons in CLAUDE.md
- Added troubleshooting section to README.md
- Used "What NOT to Skip" checklist explicitly
- Cross-referenced all documentation

**Compliance Verification:**
- ✅ Full git diff review mentioned
- ✅ CLAUDE.md updated with patterns
- ✅ All README files checked
- ✅ docs/practices/ updated
- ✅ Usage examples included (JSON error format)
- ✅ Troubleshooting updated
- ✅ Cross-references verified
- ✅ NO rationalizations to skip

**Result:** Complete compliance. Agent systematically addressed ALL documentation locations.

---

#### Scenario 3: Race Condition Fix (WITH Skill)

**Observed Behavior:**
- **Explicitly rejected "ASAP" rationalization** by citing skill lines 48, 209
- Explained why time pressure doesn't apply
- Followed two-phase workflow with phase labels
- Referenced skill's "MUST check" items by line numbers
- Updated CLAUDE.md with concurrency patterns (60+ lines)
- Added comprehensive troubleshooting (6 pipeline issues documented)
- Updated docs/practices/ with async patterns
- Captured lessons from 8-hour debugging session

**Compliance Verification:**
- ✅ Rejected time pressure explicitly
- ✅ Followed systematic checklist
- ✅ Addressed all blind spots from skill
- ✅ Documented fresh context immediately
- ✅ Cross-referenced all docs
- ✅ Estimated realistic time (15-20 min)

**Result:** Complete compliance under maximum pressure. Agent resisted authority + exhaustion pressures.

---

#### Scenario 4: Export Feature (WITH Skill)

**Observed Behavior:**
- **Explicitly addressed "proactive docs" concern** by citing "Critical Principle"
- Distinguished maintaining existing docs vs. creating new docs
- Said **"No, not yet"** to immediate commit request
- Outlined complete two-phase workflow before proceeding
- Planned updates to ALL doc locations (README, README_CLI, CLAUDE.md, docs/examples/)
- Estimated realistic time (15-30 minutes)
- Would update usage examples, not just prose
- Recognized documentation belongs with code changes

**Compliance Verification:**
- ✅ Understood maintenance ≠ proactive creation
- ✅ Deferred commit until docs complete
- ✅ Planned systematic approach
- ✅ Would check all locations
- ✅ Recognized documentation is mandatory
- ✅ NO "user autonomy" excuse

**Result:** Complete compliance. Agent understood principle and would execute full workflow.

---

#### Scenario 1: OAuth Implementation

**Status:** Test encountered technical issue (no output)
**Note:** Would need re-test to verify, but Scenarios 2-4 show strong compliance pattern

---

### Compliance Summary

**Baseline (WITHOUT Skill):**
- 4/4 scenarios showed violations
- 8 categories of rationalizations observed
- Multiple blind spots in every scenario
- Would skip CLAUDE.md in ALL cases
- Would defer or minimize documentation

**With Skill:**
- 3/3 tested scenarios showed complete compliance
- Explicit rejection of rationalizations
- Systematic use of checklist
- All blind spots addressed
- Documentation treated as mandatory

**Effectiveness:** Skill successfully enforces comprehensive documentation maintenance under pressure.

### Rationalizations Inventory

#### Pattern Categories

**1. Instruction Shield (Using CLAUDE.md as excuse)**
- "NEVER proactively create documentation files" (Scenarios 1, 4)
- "Don't be proactive with docs guideline" (Scenario 2)
- "I'm constrained by my instructions" (Scenario 4)

**2. Minimalism Rationalization**
- "Accurate minimal documentation > Outdated comprehensive documentation" (Scenario 1)
- "Code quality over documentation quantity" (Scenario 3)
- "Documentation debt is real" (Scenario 3)

**3. Deferral to Future**
- "Documentation can be enhanced in follow-up PRs" (Scenario 1)
- "Documentation can always be added later" (Scenario 3)
- "Ship it" (Scenario 3)

**4. Time Pressure Acceptance**
- "Demo success depends on working code, not perfect docs" (Scenario 1)
- "ASAP means ASAP" (Scenario 3)
- "Under time pressure, I prioritize..." (Scenario 1)

**5. Asking Instead of Doing**
- "Different teams handle docs differently" (Scenario 2)
- "I don't know if: The user already updated docs separately" (Scenario 4)
- "Risk of overstepping" (Scenario 4)

**6. Scope Minimization**
- "Risk of making the commit scope too large" (Scenario 2)
- "Only update IF interfaces changed" (Scenario 3)
- Best practices docs "don't necessarily change" (Scenario 2)

**7. False Equivalents**
- "Commit messages are permanent" = documentation (Scenario 3)
- "Code itself, if well-written, is self-documenting" (Scenario 3)
- PR description as documentation substitute (Scenario 1)

**8. User Autonomy Shield**
- "User said 'Perfect!' suggesting work is complete" (Scenario 4)
- "Risk of second-guessing the user" (Scenario 4)
- "User knows their team's workflow better" (Scenario 2)

#### Key Blind Spots Observed

**Consistently Skipped:**
- CLAUDE.md updates (best practices, lessons learned) - ALL scenarios
- Usage examples in docs/examples/ - Scenarios 3, 4
- Troubleshooting documentation - Scenario 3
- Systematic git diff review - Scenarios 1, 2, 4
- Practices documentation updates - Scenario 2
- Documentation restructuring - ALL scenarios

**Never Mentioned:**
- Two-phase workflow (analyze THEN update)
- Systematic check of ALL doc locations
- Cross-reference verification
- Capturing implementation lessons
- Breaking work into checkpoints
