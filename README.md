# ascii-vision

[![CI](https://github.com/GrigorTonikyan/ascii-vision/workflows/CI/badge.svg)](https://github.com/GrigorTonikyan/ascii-vision/actions)

A TUI CLI app to convert camera feed into ASCII vision in real time.

## Features

- **Real-time camera capture** - Captures video from your webcam
- **ASCII conversion** - Converts camera feed to ASCII art using different
  character sets
- **Multiple character sets** - Dense, Simple, Blocks, and Minimal character
  sets
- **Color support** - Optional color ASCII output using terminal colors
- **Interactive controls** - Real-time switching between modes and settings
- **Scalable output** - Adjustable scale factor for ASCII output
- **Performance monitoring** - Built-in FPS counter

## Controls

- **SPACE** - Toggle camera on/off
- **C** - Toggle color mode
- **S** - Next character set
- **A** - Previous character set
- **+** - Increase scale
- **-** - Decrease scale
- **Q** - Quit application

## Installation

```bash
git clone https://github.com/GrigorTonikyan/ascii-vision.git
cd ascii-vision
cargo build --release
```

## Usage

```bash
./target/release/ascii-vision
```

### Command Line Options

```bash
ascii-vision [OPTIONS]

Options:
  -t, --tick-rate <FLOAT>     Tick rate, i.e. number of ticks per second [default: 30.0]
  -f, --frame-rate <FLOAT>    Frame rate, i.e. number of frames per second [default: 20.0]
  -h, --help                  Print help
  -V, --version               Print version
```

## Configuration

The application can be configured via a config file located at:

- Linux: `~/.config/ascii-vision/config.json5`

### Example Configuration

```json5
{
  keybindings: {
    Home: {
      "<q>": "Quit",
      "<space>": "ToggleCamera",
      "<c>": "ToggleColor",
      "<s>": "NextCharacterSet",
      "<a>": "PreviousCharacterSet",
      "<+>": "IncreaseScale",
      "<->": "DecreaseScale",
    },
  },
  camera: {
    default_camera_index: 0,
    fps: 30.0,
    width: 640,
    height: 480,
  },
}
```

## Character Sets

1. **Dense** - `@#S%?*+;:,.` (12 characters, highest detail)
2. **Simple** - `@#*+-.` (7 characters, balanced)
3. **Blocks** - `█▉▊▋▌▍▎▏` (9 Unicode block characters)
4. **Minimal** - `█▓▒░` (5 characters, lowest detail)

## Requirements

- Rust 1.82+ (Rust 2024 Edition)
- A webcam or other video capture device
- Terminal with Unicode support (for block characters)
- Linux: v4l2 compatible camera

## Architecture

The application is built with a modular architecture:

- **TUI Framework**: Built on `ratatui` for terminal user interface
- **Camera Capture**: Uses `nokhwa` for cross-platform camera access
- **ASCII Conversion**: Custom algorithms for image-to-ASCII conversion
- **Action System**: Event-driven architecture with typed actions
- **Component System**: Modular UI components with lifecycle management

## Performance

- **Real-time performance**: 20 FPS smooth video display with optimized
  processing
- **Responsive controls**: Immediate response time (<50ms with 30Hz tick rate)
- **Optimized conversion**: 75% CPU reduction through single-pixel sampling
- **Smart frame handling**: Action prioritization and intelligent frame dropping
- **Quality-performance balance**: Triangle filtering for optimal processing
  speed
- **Adaptive resolution**: Automatic scaling to fit terminal dimensions
- **Memory efficient**: Frame throttling and optimized data structures
- **Hardware synchronization**: Proper camera state management prevents LED
  issues

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass: `cargo test`
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file
for details.
