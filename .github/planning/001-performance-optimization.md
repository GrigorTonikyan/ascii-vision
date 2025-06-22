# [Performance Optimization] – ASCII Vision Performance Improvements

```yaml
status: completed
```

## Feature Description

**Scope of Files Affected:**

- `src/cli.rs` - Update default rates
- `src/app.rs` - Optimize frame processing
- `src/ascii.rs` - Improve conversion efficiency
- `src/camera.rs` - Add frame rate limiting
- `src/tui.rs` - Optimize event handling

**Context / Background:** The ASCII vision app is experiencing significant lag
in controls (taking seconds to respond) and slow refresh rates. The current
implementation has several performance bottlenecks:

- Very low tick rate (4Hz) causing slow control response
- Synchronous ASCII conversion blocking the main loop
- No frame rate limiting on camera capture
- Expensive image filtering operations

**Purpose & Goals:**

- Improve control responsiveness to near real-time
- Increase frame rate for smoother video display
- Optimize ASCII conversion pipeline
- Reduce CPU usage through better algorithms

**Expected Outcome / Deliverable:**

- Controls respond within 100ms or less
- Smooth 30+ FPS video display
- Reduced CPU usage
- Maintained visual quality

## Work Breakdown

- [x] stage-01/task-01/step-01: Increase default tick rate from 4Hz to 30Hz for
      responsive controls
- [x] stage-01/task-01/step-02: Optimize frame rate to reasonable 30 FPS default
- [x] stage-01/task-02/step-01: Replace expensive Lanczos3 filtering with faster
      Nearest neighbor
- [x] stage-01/task-02/step-02: Add frame skipping to prevent processing backup
- [x] stage-01/task-03/step-01: Add camera frame rate limiting in capture loop
- [x] stage-01/task-03/step-02: Implement async frame processing to prevent
      blocking
- [x] stage-01/task-04/step-01: Optimize ASCII conversion by caching character
      lookup tables
- [x] stage-01/task-04/step-02: Use faster brightness calculation
