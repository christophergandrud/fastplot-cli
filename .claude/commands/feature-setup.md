# Feature Setup Command

You are tasked with setting up a new feature development workflow. This command automates the initial setup for working on a new feature.

## Input
The user will provide a feature description after the command, like: `/feature-setup "Add dark theme support"`

## Tasks to Complete

1. **Create Feature Branch**
   - Extract a clean branch name from the feature description (lowercase, hyphens for spaces, remove special chars)
   - Create and checkout a new git branch with pattern: `feature/[branch-name]`

2. **Create GitHub Issue**
   - Use the `gh` command to create a new issue
   - Title should be the feature description
   - Body should include:
     - Feature description
     - Implementation checklist placeholder
     - Testing checklist placeholder
     - Documentation checklist placeholder

3. **Generate Implementation Plan**
   - Analyze the codebase to understand the current architecture
   - Survey open source solutions to implementing similar features
   - Break down the feature into logical implementation steps
   - **Always prioritize simple and modular solutions**
   - Consider dependencies, testing requirements, and documentation needs
   - Present a structured plan with:
     - Overview of changes needed
     - Step-by-step implementation tasks (favoring modularity)
     - Files that may need modification
     - Simple, clean interfaces between components
     - Testing strategy
     - Documentation requirements

## Output Format
After completing the setup:
1. Confirm branch creation
2. Provide GitHub issue URL
3. Present the implementation plan
4. Ask user to review the plan before proceeding

## Important Notes
- Do NOT start implementing the feature
- Only set up the development environment and planning
- Wait for user approval of the plan before any code changes
- Use TodoWrite tool to track the setup process