## Why

Navigating the timeline with left/right arrow keys currently moves one hour per key press. Jumping many hours ahead or behind is tedious—holding the key repeats at a constant rate. Adding acceleration when the key is held makes long-distance navigation feel responsive without sacrificing precision for small adjustments.

## What Changes

- Track how long left/right navigation has been sustained in the same direction.
- After an initial grace period of single-step increments, increase the step size progressively up to a configurable maximum.
- Reset the acceleration state when the key is released or the direction changes.
- Each rendered frame still applies the accumulated offset to `hour_offset`, so the existing debounce/render logic stays intact.

## Capabilities

### New Capabilities
- `accelerated-nav`: Key-hold acceleration for horizontal (left/right) timeline navigation with configurable ramp-up and maximum speed.

### Modified Capabilities

## Impact

- `src/tui/event.rs` — event loop gains acceleration state tracking and variable step size for left/right keys.
- `src/tui/app.rs` — App struct may gain fields to persist acceleration state across loop iterations (or state can remain local to the event loop).
- No new external dependencies.
- No config file changes required (acceleration parameters are compile-time constants).
- No breaking changes to existing keybindings or behavior—single taps still move exactly 1 hour.
