---
applyTo: '**'
description: Security best practices for UPM development and operations
---

## General Security Principles

- Double-check everything and backup all data before making changes to OS configs or system files
- Always ask for confirmation before executing potentially destructive commands
- Before making external changes ensure there are backups
- Validate all user inputs
- Be cautious with file system operations

## Input Validation

- Sanitize all user inputs before processing
- Validate file paths to prevent directory traversal attacks
- Check package names and versions for malicious patterns
- Implement proper error handling without exposing sensitive information

## System Operations

### File System Safety

- Always check file permissions before operations
- Use temporary directories for intermediate files
- Clean up temporary files and directories after use
- Validate file paths are within expected boundaries

### Command Execution

- Never execute commands with user input directly
- Use parameterized command execution
- Validate all command arguments
- Log security-relevant operations for audit trails

## Package Manager Security

- Verify package sources and integrity when possible
- Warn users about potentially unsafe operations
- Implement rate limiting for package operations
- Cache validation results to prevent repeated unsafe operations

## Configuration Security

- Store sensitive configuration data securely
- Use environment variables for secrets
- Never log or expose sensitive configuration values
- Implement proper access controls for configuration files

## Error Handling

- Provide helpful error messages without exposing system internals
- Log errors appropriately without sensitive data
- Implement graceful degradation for security failures
- Never expose stack traces or internal paths to end users
