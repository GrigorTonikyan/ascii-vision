---
applyTo: '**'
description: Structured approach to feature planning and development tracking
---

## Development Plan Schema

Follow this structure precisely when drafting a plan. Each `.md` file defines one feature and  
must live in `.github/planning/`.

### File Naming & Location

- Create under: `./.github/planning/`
- Format: `[NNN]-[feature-name].md` (e.g., `042-user-auth.md`)
- Move completed plans to: `./.github/planning/completed/`

### Required Sections (in order)

#### 1. Title

- Format: `# [Issue #NNN] – [Descriptive Feature Name]`

#### 2. Status

Below the title:

```yaml
status: draft
# OR
status: in-progress (stage-XX/task-YY/step-ZZ)
# OR
status: completed
```

- If `completed`, move file to `/completed/`

#### 3. Feature Description

Subsections:

- **Scope of Files Affected:** List files/directories
- **Context / Background:** Explain the need
- **Purpose & Goals:** Bullet objectives
- **Expected Outcome / Deliverable:** Final artifacts
- **Requirements / Specifications:** Constraints, APIs, etc.

#### 4. Work Breakdown

- Use hierarchy: `stage → task → step`
- Format: `stage-XX/task-YY/step-ZZ`
- Each `step` is atomic and checkable:

```markdown
- [ ] stage-01/task-01/step-01: Describe the step
```

## Completion Checklist

- File in correct folder and format
- All 4 major sections are present
- Numbering is zero-padded
- Steps are checkable via `- [ ]`
- Status reflects actual progress
- Completed plans moved to `/completed`

## Plan Management Commands

### Create Development Plan

Create detailed development plan in a dedicated file within the `./.github/planning` directory.

- Filename: `[nnn]-[featurename].md` where `nnn` is the issue number and `featurename` is a  
  descriptive name for the feature
- Must include all required sections as specified above
- Each step should be atomic and independently completable

### Continue Development Plan

Continue development on existing plan file:

- Identify any new tasks or steps that need to be added if necessary
- Ensure that all ongoing tasks are properly tracked and updated
- Update status to reflect current progress

### Update Development Plan

Update the development plan to reflect current project status:

- Mark completed tasks appropriately
- Update any ongoing tasks with current progress
- Ensure that each step in the plan remains atomic and clear

### Review Development Plan

Review development plan for quality and completeness:

- Ensure that all completed tasks are properly marked
- Provide feedback on any unclear or ambiguous steps
- Suggest improvements or adjustments to the plan as necessary
