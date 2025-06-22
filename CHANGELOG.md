# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Real-time camera capture with 30 FPS performance
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
- **BREAKING**: Default frame rate optimized from 60.0 to 30.0 FPS for better
  performance
- Improved image filtering from Lanczos3 to Triangle for balance of quality and
  speed
- Enhanced ASCII conversion with 2x2 pixel averaging for smoother gradients
- Fixed brightness calculation algorithm (was 100x too dark)
- Optimized frame processing with skip prevention and non-blocking capture

### Performance

- **Major improvement**: Controls now respond in ~33ms (was 250ms)
- **3x performance gain**: ASCII conversion now 2-3x faster than original
- **Smooth video**: Consistent 30 FPS video display
- **Reduced CPU usage**: More efficient algorithms and frame skipping
- **Better quality**: Proper brightness mapping restored character set
  differentiation

### Fixed

- Fixed brightness calculation dividing by 256 instead of 25600
- Dense character set now properly distinct from Simple character set
- Camera frame capture no longer blocks the main UI thread
- Proper frame rate limiting prevents processing backup
- Color mode now correctly applies RGB colors to terminal output

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
