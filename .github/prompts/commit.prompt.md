---
mode: agent
---

# Conventional Commit and Versioning

## Purpose

Analyze the codebase and recent changes. Create a commit message that strictly follows  
[Conventional Commits](https://www.conventionalcommits.org/) standards. Ensure the commit  
is precise and includes all important details.

## Pre-Commit Analysis

- [ ] Review all recent codebase changes (staged files, modifications)
- [ ] Identify the primary type of change (feat, fix, chore, docs, refactor, etc.)
- [ ] Determine if changes include breaking changes
- [ ] Check if version bump is required per [Semantic Versioning](https://semver.org/)
- [ ] Update package.json version if changes warrant it

## Commit Message Format

```text
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types

- `feat`: New feature for the user
- `fix`: Bug fix for the user
- `docs`: Documentation changes
- `style`: Formatting changes (no code change)
- `refactor`: Code change that neither fixes bug nor adds feature
- `test`: Adding missing tests or correcting existing tests
- `chore`: Changes to build process or auxiliary tools

### Breaking Changes

If changes introduce breaking changes, include `BREAKING CHANGE:` in the footer:

```text
feat(api): add new authentication system

BREAKING CHANGE: Authentication format changed, old tokens will no longer work
```

## Semantic Versioning Requirements

**IMPORTANT**: Update `package.json` version according to semantic versioning rules:

### Version Bump Rules

- **MAJOR (X.0.0)**: Breaking changes

  - Any commit with `BREAKING CHANGE:` in footer
  - API changes that break backward compatibility
  - Removal of deprecated features

- **MINOR (0.X.0)**: New features (backward compatible)

  - `feat:` commits that add new functionality
  - New commands, options, or capabilities
  - Enhancements that don't break existing usage

- **PATCH (0.0.X)**: Bug fixes and maintenance
  - `fix:` commits that resolve bugs
  - `chore:`, `docs:`, `style:`, `refactor:`, `test:` commits
  - Performance improvements without new features

### Version Update Process

1. **Identify the change type** from commit analysis
2. **Update package.json version** accordingly:

   ```bash
   # For new features
   bun version minor

   # For bug fixes
   bun version patch

   # For breaking changes
   bun version major
   ```

3. **Include version update in the same commit** as the changes
4. **Tag the commit** with the new version after pushing

## Quality Checklist

- [ ] Summarize changes accurately and concisely
- [ ] Use the correct Conventional Commit type
- [ ] Include scope when changes affect specific components
- [ ] Add breaking change indicators if applicable
- [ ] Do not omit any important details from the changes
- [ ] **Update package.json version** if changes warrant it (feat = minor, fix = patch, BREAKING = major)
- [ ] Ensure commit message is under 72 characters for the subject line
- [ ] Include version update in commit if package.json was modified

## Example

```text
feat(cli): add interactive package manager selection

- Implemented interactive prompts for package manager selection
- Added validation for package manager availability
- Updated documentation with new interactive features
- Bump version to 2.5.0 for new feature

Closes #123
```

### Version Bump Examples

```text
# Minor version bump for new feature
feat(commands): add new 'validate' command for config checking

- Added config validation command
- Implemented schema-based validation
- Version bumped to 2.5.0

# Patch version bump for bug fix
fix(detector): resolve package manager detection on Windows

- Fixed path resolution issue on Windows systems
- Improved error handling for missing executables
- Version bumped to 2.4.1

# Major version bump for breaking change
feat(config)!: redesign configuration file format

BREAKING CHANGE: Configuration file format changed from JSON to YAML.
Old .upm-config.json files will need migration to .upm-config.yaml

- Version bumped to 3.0.0
```
