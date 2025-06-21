---
mode: agent
description: Create and manage structured development plans for features and tasks
---

# Development Planning Workflow

## Create Development Plan

Create a detailed development plan in `./.github/planning/[NNN]-[feature-name].md`

### Plan Structure

```markdown
# [Issue #NNN] – [Descriptive Feature Name]

status: draft

## Feature Description

### Scope of Files Affected

- List all files and directories that will be modified

### Context / Background

- Explain why this feature is needed
- Provide background information

### Purpose & Goals

- Clear objectives for the feature
- Success criteria

### Expected Outcome / Deliverable

- What will be delivered
- How success will be measured

### Requirements / Specifications

- Technical requirements
- Constraints and limitations
- API specifications if applicable

## Work Breakdown

### stage-01: [Stage Name]

- [ ] stage-01/task-01/step-01: [Atomic step description]
- [ ] stage-01/task-01/step-02: [Atomic step description]

### stage-02: [Stage Name]

- [ ] stage-02/task-01/step-01: [Atomic step description]
```

### Planning Guidelines

- Each step must be atomic (independently completable)
- Use format: `stage-XX/task-YY/step-ZZ`
- Number with zero-padding (01, 02, 03...)
- Steps should be checkable with `- [ ]`

## Plan Management

### Update Status

Update plan status as work progresses:

```yaml
status: in-progress (stage-02/task-01/step-03)
```

### Mark Completion

When plan is complete:

1. Change status to `completed`
2. Move file to `./.github/planning/completed/`

## Plan Types

### Feature Plans

- New functionality development
- Major feature additions
- User-facing improvements

### Refactoring Plans

- Code reorganization
- Architecture improvements
- Technical debt reduction

### Bug Fix Plans

- Complex bug investigations
- Multi-file bug fixes
- Systematic issue resolution

## Review Process

### Plan Review Checklist

- [ ] All required sections are present
- [ ] File naming follows convention
- [ ] Steps are atomic and clear
- [ ] Numbering is consistent and zero-padded
- [ ] Status accurately reflects progress
- [ ] Requirements are well-defined
