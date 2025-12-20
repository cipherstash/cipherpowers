Best Practices for AGENTS.md and CLAUDE.md
Table of Contents
Introduction to AGENTS.md vs CLAUDE.md
Key Structure and Sections
Guidance on Concise, Relevant Content
Hierarchical Memory Files
Tools and Skills to Maintain Context
Multi-Agent Context Tips
Final Recommendations and Takeaways
Introduction to AGENTS.md vs CLAUDE.md
AI coding assistants use special memory files to understand your project context. CLAUDE.md originated as Anthropic Claude's project instruction file: when you run Claude in a codebase, it automatically loads any CLAUDE.md and sends its content to the model as context
pnote.eu
. Many coding agents have adopted similar files with different names (e.g. Copilot’s instructions file, Cursor’s rules file), leading to fragmentation
pnote.eu
. The community is now converging on AGENTS.md as a unified, tool-agnostic standard
pnote.eu
pnote.eu
. In short, AGENTS.md is an open-standard context file recognized by multiple AI systems (future-proof and not tied to a single platform)
solmaz.io
, whereas CLAUDE.md was specific to Claude Code. Think of AGENTS.md like a README for AI agents. Just as README.md guides human developers (project intro, installation, usage), AGENTS.md guides coding AI on how to work with the codebase
addozhang.medium.com
. It provides detailed info the AI needs: environment setup, build/test instructions, coding conventions, etc. The goal is to onboard the AI model to what the project is, why it exists, and how to work with it
humanlayer.dev
. Importantly, AGENTS.md content is injected into every conversation the agent has in that repo, so it’s the primary way to give the model persistent knowledge of the project context
humanlayer.dev
. Today, nearly all major coding assistants support AGENTS.md out-of-the-box as the default memory file format
addozhang.medium.com
 (many still recognize their legacy formats as well, for compatibility).
Key Structure and Sections
What should an AGENTS.md/CLAUDE.md contain? A well-structured file typically covers several key sections to comprehensively introduce the project to the AI. Here are common components and organization strategies:
Project Overview and Purpose: Begin with a high-level summary of the project – what the software does and why it exists
humanlayer.dev
. Describe the tech stack and major architecture or modules. This gives the AI context about the domain and goals of the codebase (e.g. “This is a web app for X, composed of front-end React and back-end Node services”). Essentially, give a map of the codebase and the function of each part
humanlayer.dev
.
Setup, Build, and Test Instructions: Include instructions on how to run, build, and test the project. For example, specify build commands, startup scripts, test suite commands, or any special environment setup steps (e.g. “Use bun instead of npm to run the app” or “Run make test for the test suite”)
humanlayer.dev
. This ensures the agent knows how to execute and verify changes. Also note any required development tools or dependencies and how to install or use them.
Code Style and Guidelines: Summarize any important coding conventions or style guidelines the project follows (naming conventions, formatting rules, architectural patterns). However, keep this high-level – only the most essential rules that the AI should always follow (more on keeping this concise in the next section). Example: “Follow PEP8 style for Python.” Many teams include repository etiquette or contribution guidelines here as well (e.g. commit message format or code review rules)
pnote.eu
.
Key Files or Components: Highlight core parts of the repository structure. You might list critical directories or files with brief descriptions of their purpose
docs.github.com
docs.github.com
. For instance: “/backend/ – Node.js API server code; /frontend/ – React UI code; /configs/ – configuration files, etc.” This acts as a quick reference so the AI knows where to find certain functionality. Mention any utility scripts or core libraries that are central to the project
pnote.eu
.
Testing and Quality: Provide guidance on running tests and ensuring code quality. This can overlap with build instructions but might include specifics like “Run npm run lint before committing” or “All new features must include unit tests – see /tests/ directory”
docs.github.com
docs.github.com
. Also note if the project has CI checks that the AI should satisfy (failing tests, linters, etc.).
Security or Special Considerations: If applicable, mention anything the AI should be cautious about, such as secrets management, regulatory requirements, or performance-critical sections. For example, “Do not log user passwords” or “This module handles payments, so ensure all compliance tests pass.” This section is optional but can be important if your project has critical dos and don’ts.
These sections need not be lengthy prose. Many teams use bullet points or short paragraphs under each heading for clarity (e.g. a bulleted list of build steps, or key guidelines)
docs.github.com
addozhang.medium.com
. The goal is to give enough detail that the AI has a clear understanding of the project’s context and norms without overwhelming it with unnecessary info. In practice, a good AGENTS.md might be a few dozen to a few hundred lines covering the above areas in a concise way. Anthropic’s guidance (for CLAUDE.md) suggests including things like common build commands, core utilities, style guides, testing instructions, repo etiquette, and environment setup
pnote.eu
 – effectively the same categories listed above.
Guidance on Concise, Relevant Content
When it comes to instruction files, less is more. Large language models have limited attention and will ignore or forget instructions that are not consistently relevant
humanlayer.dev
humanlayer.dev
. Claude’s own system will actively tell the model not to use the context file unless it’s highly relevant to the current task
humanlayer.dev
. This means stuffing the file with every possible rule or command can backfire: if the model judges most of it as unrelated noise, it may skip using the file entirely
humanlayer.dev
. To avoid this, make sure every line in your AGENTS.md is broadly applicable to any work done in the repo. Keep the content focused and universally relevant to the project. Do not include lengthy instructions for edge cases or one-off scenarios that won’t matter in most sessions
humanlayer.dev
. For example, including a detailed guide on migrating the database schema might only be relevant when doing database work – if the AI is working on UI code, that’s wasted context and can cause it to ignore the whole file. Instead, include only the core, always-important info in the main memory file. You can handle special cases with other methods (see “Hierarchical Memory Files” below). Likewise, avoid copying large snippets of code or config into the instructions. If specific code examples are needed, it’s often better to refer to them by file path or give a brief description, rather than pasting 50 lines of code in your AGENTS.md. Detailed code will consume tokens and quickly become outdated. One strategy is to point to authoritative sources (e.g. “See config.yml for the default configuration values” or “Refer to Utils.java for common helper functions”)
humanlayer.dev
. Optimize instruction count: Research indicates that as you increase the number of separate instructions or rules, model performance in following them degrades
humanlayer.dev
. Every additional rule slightly dilutes the model’s focus on the others. Smaller models are especially prone to this decline, but even larger frontier models show linear drop-offs as instruction count grows
humanlayer.dev
. In practical terms, it’s usually better to have a handful of critical guidelines than a laundry list of dozens of minor points. Aim to write as few instructions as reasonably necessary while still covering the key points
humanlayer.dev
. A widely shared guideline is to keep the file under ~300 lines at most, and many find 100-200 lines or fewer is optimal
humanlayer.dev
. (For instance, one engineering team reports their root CLAUDE.md is under 60 lines
humanlayer.dev
.) Brevity forces you to include only what truly matters, which helps the model focus on the most important context
humanlayer.dev
. In summary, be concise and relevant above all. Use clear, direct language for instructions. Remove any content that isn’t generally applicable to most coding tasks in the repo
humanlayer.dev
. The payoff will be that the agent actually pays attention to your memory file, instead of skipping it due to bloat. Remember, the context window is precious – fill it with high-value information rather than exhaustive minutiae.
Hierarchical Memory Files
For complex projects, you may find it challenging to fit all relevant guidance into a single short file. This is where hierarchical or layered memory files come in. The idea is to use multiple levels or files of context, revealing additional details only when needed – often called progressive disclosure
humanlayer.dev
. One approach is to maintain a primary AGENTS.md at the root with the universal info, and then have supplementary files for specific domains or components. For example, you might create a folder like agent_docs/ (or use subdirectory-specific files) containing documents such as building_the_project.md, running_tests.md, code_conventions.md, service_architecture.md, etc.
humanlayer.dev
. Each of those can hold in-depth instructions on that particular topic. Your main AGENTS.md would then refer the agent to these resources when appropriate – for instance, listing the documents and saying “If working on database changes, see agent_docs/database_schema.md for guidance”
humanlayer.dev
. This way, the detailed instructions are available to the AI but not loaded unless they’re relevant to the task at hand. Claude Code’s creators recommend exactly this pattern: instead of cramming everything into CLAUDE.md, keep task-specific or module-specific guidance in separate files and have the agent fetch them as needed
humanlayer.dev
. You can instruct the AI to decide which (if any) of those files to read based on the context of its current task
humanlayer.dev
. In practice, an agent could be prompted (manually or via automation) to open the relevant file for more details once it knows what it’s working on. This keeps the initial prompt lighter and focused. Another form of hierarchy is to use directory-level AGENTS.md files in a monorepo or multi-project repository. For example, after migrating to the AGENTS.md standard, you might have: one top-level AGENTS.md with general info, plus an apps/AGENTS.md for frontend-specific instructions, and a backend/AGENTS.md for backend-specific instructions
solmaz.io
. When the AI is operating in a particular subfolder, it can load that folder’s AGENTS.md for context on that sub-project. This segmentation means the agent sees instructions scoped to the part of the codebase it's working on, which improves relevance. Maintaining multiple instruction files does require discipline (ensuring consistency), but it allows very targeted context injection. Whichever method you use, avoid duplication of information across files. Use the primary file to point to others rather than copying their content. And if you have multiple files, update all of them as needed so they don’t contradict each other. The hierarchical approach should simplify context, not create confusion. Think of the top-level file as the table of contents or executive summary, with deeper dives available on demand. Also note: when referencing other files, prefer to describe what’s in them or use file/line references rather than copy-pasting large sections
humanlayer.dev
. For example, instead of including a whole testing script, you might say “the test procedure is defined in TESTING.md – see that file for details” and maybe highlight key lines. This ensures there is a single source of truth for that information and reduces maintenance burden.
Tools and Skills to Maintain Context
Not every rule or practice needs to live in the prompt context. Modern AI coding assistants often support external tools, skills, or hooks that can shoulder some of the burden of guidance. Leveraging these can keep your AGENTS.md lean while still enforcing important standards. One big example is coding style and linting. As tempting as it is to put your entire style guide into the AI’s instructions, it's usually counterproductive. “Never send an LLM to do a linter's job,” as one expert put it
humanlayer.dev
. Style rules and formatting conventions tend to be numerous and not all equally crucial; listing them all will bloat the context and distract the model
humanlayer.dev
. Moreover, an AI can often infer your coding style by examining the existing codebase – they are in-context learners, so if your code consistently follows certain patterns, the AI will likely pick that up without being explicitly told
humanlayer.dev
. Instead of overloading AGENTS.md with style nits, rely on actual linters/formatters to handle that. Most agent platforms allow you to integrate automated checks. For instance, Claude Code supports hooks (like a sessionStart or “stop” hook) that can run your formatter and linter, then feed any errors back to the AI to fix
humanlayer.dev
. Similarly, you could configure a script to run tests or static analysis whenever the agent proposes changes, rather than trying to teach the AI every test expectation upfront. Using these tools means the AI doesn’t need to memorize all rules – it just reacts to concrete feedback (failing tests, linter output) and corrects accordingly. This division of labor makes the AI’s job easier and your context file smaller. Another feature is skills or commands provided by AI platforms. For example, Anthropic’s Claude has a concept of Skills, and GitHub Copilot’s agents can use the Model Context Protocol (MCP) to call external tools
docs.github.com
. You might have a skill for searching the codebase, one for running the application, one for retrieving docs, etc. Rather than writing instructions like “If you need to do X, open file Y and look at function Z,” you could just empower the agent with a search tool or a knowledge base. In your AGENTS.md, you then simply instruct the agent at a high level – e.g. “you have a search tool, use it to find relevant code” – instead of enumerating every possible file it might need. This maintains context by giving the AI the ability to fetch info when needed, instead of pre-loading all that info. Example – Enforcing style via tools: Instead of listing 20 style rules in the memory file, one can configure a slash command or button that, when invoked, provides the agent with the style guidelines (or runs a formatter) just at the moment it’s needed
humanlayer.dev
. The HumanLayer blog suggests creating a slash command that supplies code guidelines and points the agent to the specific changed lines or git status output, so it can review formatting only on the code it just wrote
humanlayer.dev
. This way, the instructions for formatting aren’t persistently taking up space in every conversation – they’re only brought in when formatting is actually relevant. In summary, use the platform’s capabilities to your advantage:
Automate repetitive checks: Use CI, linters, tests, and hooks to catch issues. Have the agent fix issues from these results, rather than front-loading all rules.
On-demand knowledge: Use skills or commands to fetch context (documentation, examples, etc.) as needed, instead of writing exhaustive documentation into the prompt.
Delegate to determinism: Wherever possible, let deterministic tools (compilers, formatters, static analyzers) handle tasks – the AI can then focus on creative and complex tasks with the assurance that tooling will catch the rest
humanlayer.dev
humanlayer.dev
.
By keeping AGENTS.md focused on essential guidance and relying on tools for the rest, you maintain a cleaner context and get better performance from the AI.
Multi-Agent Context Tips
One of the main reasons for the shift to AGENTS.md is to make your project’s AI instructions universal across different assistants. If you and your team use multiple AI coding agents (say, Claude and GitHub Copilot, or others), a single AGENTS.md ensures everyone is drawing from the same playbook
pnote.eu
pnote.eu
. Here are some tips to manage context in a multi-agent environment:
Use a Single Source of Truth: Keep one primary instructions file (AGENTS.md) and have all agents reference it. You can achieve this by symlinking or including references. For example, if Claude still expects CLAUDE.md, create a symlink or stub CLAUDE.md that points to AGENTS.md so they always stay in sync
pnote.eu
solmaz.io
. Claude’s own docs suggest you can literally include the content of AGENTS.md by writing @AGENTS.md in your CLAUDE.md – the Claude harness will then load the unified file
code.claude.com
. GitHub’s Copilot also supports both naming conventions (AGENTS.md and CLAUDE.md among others) so it will pick up whichever is present
docs.github.com
. The goal is to avoid maintaining duplicate files for different agents.
Agent-Agnostic Wording: Write instructions in a neutral way that any AI agent can understand. Don’t address “Claude” or use platform-specific jargon. For instance, instead of “This file provides guidance to Claude Code,” say “...guidance to AI agents…”
solmaz.io
. Avoid relying on proprietary features in the universal file; if one agent has a special ability, you might document that separately or not at all in the shared file. Keep the shared instructions general enough that they make sense to Copilot, Claude, or any other system.
Mind the Differences: Despite the push for standardization, remember that each AI system might have its own quirks. The optimal structure for one might not be identical for another
reddit.com
. For example, one user noted that copying a Claude-optimized instruction file directly into Copilot’s instructions wasn’t as effective as having a file tailored to Copilot
reddit.com
. Copilot’s context window and prompt format differ from Claude’s, etc., so you might need to adjust how much detail to include. As a rule of thumb, keep the core content the same (project overview, key commands), but be prepared to tweak phrasing or trim length if a particular agent works better with a slightly different approach. If maintaining separate files or sections for different agents yields significantly better results, you can do so – but try to minimize divergence to reduce maintenance overhead.
Stay Updated on Support: The landscape of AI dev tools is evolving. By using the open standard file name and format, you’re future-proofing your project’s AI instructions. Keep an eye on updates from tools you use; for instance, new agents might introduce their own specialties or deprecate old formats. As of late 2025, all major coding assistants (Anthropic Claude, GitHub Copilot, OpenAI Code Interpreter, Sourcegraph’s Cody, Cursor, etc.) either support AGENTS.md natively or via backward compatibility
addozhang.medium.com
. So, adopting AGENTS.md is a safe bet for multi-agent compatibility.
In practice, teams that have transitioned to AGENTS.md often add a note in the file itself to clarify its purpose and interoperability – e.g. a blurb at the top: “Note: This project uses the open AGENTS.md standard. A symlinked CLAUDE.md is provided for compatibility with Claude Code. All agent instructions reside in this file.”
solmaz.io
. This communicates to contributors what’s going on and avoids confusion. Finally, encourage consistency among team members: if different developers use different AI tools, ensure everyone is contributing to and reading from the same AGENTS.md so knowledge doesn’t get siloed or duplicated in tool-specific files.
Final Recommendations and Takeaways
To wrap up, here are the key best practices for crafting effective AGENTS.md/CLAUDE.md files:
Onboard the AI to your project’s “Why, What, How”: Use the memory file to clearly explain the project’s purpose (why it exists), the technology and structure (what it is), and the development workflow (how to build/test and contribute)
humanlayer.dev
. This ensures the agent starts each session with crucial context.
Keep it concise and universally relevant: Every instruction should be broadly applicable. Aim for minimal but powerful guidance rather than an exhaustive manual
humanlayer.dev
. Avoid niche details that only sometimes matter – they’ll just dilute the important points.
Limit instruction count and length: Don’t overstuff the file with dozens of rules. Fewer, well-chosen instructions lead to better compliance. Try to stay under a few hundred lines at most (shorter if possible) for the entire file
humanlayer.dev
humanlayer.dev
. Use simple language and bullet points for clarity.
Use progressive disclosure of info: Instead of dumping everything in one place, link out to additional docs or maintain sub-file instructions for specific domains
humanlayer.dev
humanlayer.dev
. Let the agent fetch or ask for those details when needed. This keeps the main context focused.
Leverage tools over hardcoding rules: Offload mechanical or verbose guidelines to automated tools and hooks. Use linters, formatters, test suites, and other skills to enforce standards dynamically, rather than writing all those rules into the prompt
humanlayer.dev
humanlayer.dev
. The AI should collaborate with your tooling, not replace it.
Adopt the open standard (AGENTS.md): Unify your instructions in an agent-neutral file to work across different AI assistants
pnote.eu
addozhang.medium.com
. If you need to support a specific format like CLAUDE.md concurrently, use symlinks or includes so that there is only one canonical content to maintain
pnote.eu
code.claude.com
. This avoids drift between multiple files.
Craft it deliberately (don’t auto-generate blindly): Treat your memory file as a critical piece of project infrastructure. Spend time refining its content. Auto-generation tools can be a starting point, but always review and edit – a single poorly-thought-out line in AGENTS.md can mislead the AI in every session
humanlayer.dev
humanlayer.dev
. Given how high-impact this file is on the AI’s behavior, ensure it’s accurate and purposeful.
By following these practices, you’ll create an AGENTS.md (or CLAUDE.md) that significantly boosts your AI coding partner’s effectiveness. A concise, well-structured memory file means the agent will have the right context and guidance to produce better results consistently
humanlayer.dev
humanlayer.dev
. In turn, this leads to a virtuous cycle: the AI’s output improves, developers trust and use it more, and the guidance can be iteratively refined. Embrace AGENTS.md as a living document – update it as your project evolves and as you learn what information helps the AI the most. With a thoughtful approach, your AI assistant will truly become a knowledgeable collaborator embedded in your codebase.
Citations

AGENTS.md becomes the convention

https://pnote.eu/notes/agents-md/

AGENTS.md becomes the convention

https://pnote.eu/notes/agents-md/

AGENTS.md becomes the convention

https://pnote.eu/notes/agents-md/

AGENTS.md becomes the convention

https://pnote.eu/notes/agents-md/

CLAUDE.md to AGENTS.md Migration Guide | Onur Solmaz blog

https://solmaz.io/log/2025/09/08/claude-md-agents-md-migration-guide/

AGENTS.md: A New Standard for Unified Coding Agent Instructions | by Addo Zhang | Medium

https://addozhang.medium.com/agents-md-a-new-standard-for-unified-coding-agent-instructions-0635fc5cb759

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

AGENTS.md: A New Standard for Unified Coding Agent Instructions | by Addo Zhang | Medium

https://addozhang.medium.com/agents-md-a-new-standard-for-unified-coding-agent-instructions-0635fc5cb759

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

AGENTS.md becomes the convention

https://pnote.eu/notes/agents-md/

Best practices for using GitHub Copilot to work on tasks - GitHub Enterprise Cloud Docs

https://docs.github.com/en/enterprise-cloud@latest/copilot/tutorials/coding-agent/get-the-best-results

Best practices for using GitHub Copilot to work on tasks - GitHub Enterprise Cloud Docs

https://docs.github.com/en/enterprise-cloud@latest/copilot/tutorials/coding-agent/get-the-best-results

Best practices for using GitHub Copilot to work on tasks - GitHub Enterprise Cloud Docs

https://docs.github.com/en/enterprise-cloud@latest/copilot/tutorials/coding-agent/get-the-best-results

Best practices for using GitHub Copilot to work on tasks - GitHub Enterprise Cloud Docs

https://docs.github.com/en/enterprise-cloud@latest/copilot/tutorials/coding-agent/get-the-best-results

AGENTS.md: A New Standard for Unified Coding Agent Instructions | by Addo Zhang | Medium

https://addozhang.medium.com/agents-md-a-new-standard-for-unified-coding-agent-instructions-0635fc5cb759

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

CLAUDE.md to AGENTS.md Migration Guide | Onur Solmaz blog

https://solmaz.io/log/2025/09/08/claude-md-agents-md-migration-guide/

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Best practices for using GitHub Copilot to work on tasks - GitHub Enterprise Cloud Docs

https://docs.github.com/en/enterprise-cloud@latest/copilot/tutorials/coding-agent/get-the-best-results

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

AGENTS.md becomes the convention

https://pnote.eu/notes/agents-md/

AGENTS.md becomes the convention

https://pnote.eu/notes/agents-md/

CLAUDE.md to AGENTS.md Migration Guide | Onur Solmaz blog

https://solmaz.io/log/2025/09/08/claude-md-agents-md-migration-guide/

Claude Code on the web - Claude Code Docs

https://code.claude.com/docs/en/claude-code-on-the-web

Best practices for using GitHub Copilot to work on tasks - GitHub Enterprise Cloud Docs

https://docs.github.com/en/enterprise-cloud@latest/copilot/tutorials/coding-agent/get-the-best-results

CLAUDE.md to AGENTS.md Migration Guide | Onur Solmaz blog

https://solmaz.io/log/2025/09/08/claude-md-agents-md-migration-guide/

Agents.md vs Claude.md : r/GithubCopilot

https://www.reddit.com/r/GithubCopilot/comments/1nee01w/agentsmd_vs_claudemd/

Agents.md vs Claude.md : r/GithubCopilot

https://www.reddit.com/r/GithubCopilot/comments/1nee01w/agentsmd_vs_claudemd/

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

AGENTS.md: A New Standard for Unified Coding Agent Instructions | by Addo Zhang | Medium

https://addozhang.medium.com/agents-md-a-new-standard-for-unified-coding-agent-instructions-0635fc5cb759

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md

Writing a good CLAUDE.md | HumanLayer Blog

https://www.humanlayer.dev/blog/writing-a-good-claude-md
All Sources