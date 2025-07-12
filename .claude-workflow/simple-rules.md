# Simple Claude Agent Rules

## Core Rules (6 simple rules)

### 1. Always work in a branch, never on main
```bash
git checkout -b feature/short-description
```

### 2. Always create and close GitHub issues
```bash
gh issue create --title "Description of work"
# Link branch to issue in commits
# Close issue when work is complete
```

### 3. Always propose a plan first (3 bullet points max)
```markdown
## Plan: [title]
- Step 1
- Step 2  
- Step 3

Proceed?
```

### 4. Always propose modular designs
- Create reusable core functions (e.g. axis handling, canvas drawing)
- Look for shared components before getting started
- Build specific features using shared components
- Design for reuse across different plot types

### 5. Keep it simple
- Use what already exists
- Don't over-engineer
- Solve only what's asked

### 6. Clean up after
```bash
# After work is done
git checkout main
git merge feature/short-description
git branch -d feature/short-description
```

That's it.