# Development Planning Directory

This directory contains structured development plans for UPM features and tasks.

## Directory Structure

- `./` - Active development plans
- `./completed/` - Completed development plans

## Plan Naming Convention

Use the format: `[NNN]-[feature-name].md`

- `NNN` - Zero-padded issue number (e.g., `001`, `042`, `123`)
- `feature-name` - Descriptive kebab-case feature name

## Plan Status

Plans should include status indicators:

- `draft` - Initial planning phase
- `in-progress (stage-XX/task-YY/step-ZZ)` - Active development
- `completed` - Finished (move to `completed/` directory)

## Creating Plans

Use the [Development Planning Prompt](../copilot/prompts/development-planning.md) to create  
properly structured development plans.

## Guidelines

- Each step must be atomic (independently completable)
- Use hierarchical numbering: `stage-XX/task-YY/step-ZZ`
- Include checkboxes for progress tracking: `- [ ]`
- Move completed plans to `completed/` directory
