# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2025-01-01

### Summary

This release delivers a major performance overhaul addressing critical
responsiveness issues and camera reliability problems. The application now
provides immediate control response and stable camera operation.

### Added

- Real-time camera capture with 20 FPS performance
- Multiple ASCII character sets (Dense, Simple, Blocks, Minimal)
- Interactive controls for camera and ASCII settings
- Color ASCII support using terminal RGB colors
- Scalable output with adjustable scale factor
- Performance monitoring with FPS counter
- Camera device selection and configuration
- Error handling and recovery for camera failures
- Optimized ASCII conversion algorithms

### Changed

- **BREAKING**: Default tick rate increased from 4.0 to 30.0 Hz for responsive
  controls
- **BREAKING**: Default frame rate optimized from 60.0 to 20.0 FPS for better
  responsiveness
- Improved image filtering from Lanczos3 to Triangle for balance of quality and
  speed
- Enhanced ASCII conversion from 2x2 pixel sampling to single pixel for
  performance
- Fixed brightness calculation algorithm (was 100x too dark)
- Optimized frame processing with priority queue and frame throttling

### Performance

- **Major improvement**: Controls now respond immediately (<50ms response time)
- **75% CPU reduction**: ASCII conversion optimized from 2x2 to single pixel
  sampling
- **Smooth video**: Consistent 20 FPS video display with UI responsiveness
- **Action prioritization**: UI actions processed before camera frames
- **Frame throttling**: Intelligent frame dropping prevents processing backup

### Fixed

- **CRITICAL**: Fixed unresponsive controls after 10-20 seconds of camera
  operation
- **CRITICAL**: Fixed camera toggle failure after first off/on cycle
- **CRITICAL**: Fixed camera hardware/software state synchronization issues
- **CRITICAL**: Fixed performance degradation after failed camera stop
  operations
- Fixed camera hardware LED staying on when camera stopped in app
- Fixed brightness calculation dividing by 256 instead of 25600
- Dense character set now properly distinct from Simple character set
- Camera frame capture no longer blocks the main UI thread
- Proper frame rate limiting prevents processing backup
- Color mode now correctly applies RGB colors to terminal output
- Camera stream lifecycle properly managed with atomic stop operations
- Failed camera operations no longer leave camera in unusable state
- Code quality improvements: Fixed all Clippy warnings and linting issues

## [0.1.0] - Initial Release

### Initial Features

- Basic TUI framework with ratatui
- Camera capture integration with nokhwa
- ASCII art conversion algorithms
- Configuration system with JSON5 support
- Cross-platform compatibility (Linux, macOS, Windows)
- MIT license

### Capabilities

- Terminal-based user interface
- Real-time camera feed processing
- ASCII art generation
- Configurable keybindings
- Error handling and logging
