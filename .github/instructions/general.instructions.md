---
applyTo: "**"
description: Core development principles and standards for UPM project
---

## Development Principles

- Follow industry standards and best practices
- Ensure code is clean, readable, and well-documented
- Project structure should be modular and maintainable
- Use [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) for changelogs
- Adhere to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
- Use conventional commit messages
- Maintain a consistent coding style across the project

## Code Quality Standards

### Tools and Formatting

- Keep dependencies up-to-date and secure

### Testing Requirements

- Write unit tests for all new features and bug fixes
- Tests are important - if you see a test is missing, add it
- Test all functionality before committing changes
- Strive for high test coverage

## Development Workflow

### Version Control

- Use GitHub Actions for CI/CD
- Follow conventional commit message format
- Create meaningful commit messages that explain the "why" not just the "what"

### Root Directory Organization

**IMPORTANT**: Keep the project root clean and organized:

- **Only** `README.md` and `CHANGELOG.md` are allowed as documentation files at
  the root
- All other documentation must be placed in `.github/docs/`
- Project planning documents go in `.github/planning/`
- Technical specifications and assessments go in `.github/docs/`
- This rule ensures a clean, professional project structure

## Documentation Standards

- Keep `README.md` and `CHANGELOG.md` up-to-date with current functionality
- Document all public APIs and interfaces
- Include code examples where appropriate
- Maintain architecture documentation for complex systems
- Use clear, concise language in all documentation
