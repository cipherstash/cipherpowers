---
name: Documentation Practices
description: Maintain clear, consistent, and complete documentation with proper formatting, structure, and examples to help users understand the code.
when_to_use: when creating or updating documentation to ensure it is well-structured, comprehensive, and user-friendly
applies_to: all projects
related_practices: development.md
version: 1.0.0
---

# Documentation practices

##  Documentation formatting and structure

1. Maintain consistent documentation style:
  - Use clear headings and sections
  - Include code examples where helpful
  - Use status indicators (✅, ⚠️, ❌) consistently
  - Maintain proper Markdown formatting
  - Use sentence case for all titles and headings

2. Ensure documentation completeness:
  - Cover all implemented features
  - Include usage examples
  - Document API changes or additions
  - Include troubleshooting guidance for common issues

3. Help users understand the code
  - Provide clear, concise examples
  - Explain usage
  - Structure logically, grouping related topics together

4. Use readable formatting patterns:
  - For definition lists or titled items, separate titles from descriptions:
    ```markdown
    1. **Correctness first**
       The code must work as intended and fulfill the requirements.
    ```
  - This makes titles stand out and improves scannability

## README guidelines

The project README.md should include:
  - a short paragraph to explain what and why
  - getting started
  - essential dependencies
  - task and/or command reference
  - practical code usage examples
  - trouble shooting
  - links and cross-references to related documentation
  - project structure
  - tech stack
  - license

- keep the README concise
  - consider splitting large README files into several smaller files
  - link to specialised or focussed docs for deep coverage of specific topics
  - additional README file names should use the prefix `README_*`
  - examples:
   - `README_ARCHITECTURE.md`
   - `README_CONTRIBUTING.md`
  - essentials should always go in the `README.md`
  - all additional README files should be referenced and linked in the `README.md`
  - good candidates for a dedicated README file
    - deep coverage of specific topics
    - architecture
    - design notes
    - internal details
    - contribution guidelines
