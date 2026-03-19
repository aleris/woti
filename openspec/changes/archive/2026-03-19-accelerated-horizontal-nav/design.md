## Context

The TUI app lets users scrub through a 24-hour timeline with left/right arrow keys. Each key event (only `KeyEventKind::Press` is handled today) adjusts `pending_h_offset` by ±1 hour, which is flushed to `hour_offset` on the next render after the 50 ms debounce.

Holding a key down currently does nothing extra because `KeyEventKind::Repeat` events are ignored. Navigating 12+ hours requires many individual taps.

## Goals / Non-Goals

**Goals:**
- Make held left/right keys accelerate: start at 1 hour/step, ramp up to a maximum step size.
- Keep single-tap behavior unchanged (always ±1 hour).
- Acceleration resets when the key is released or direction reverses.

**Non-Goals:**
- Configurable acceleration curves via the config file (compile-time constants are sufficient).
- Acceleration for vertical (up/down) scroll — those navigate timezone rows, not hours.
- Page-jump shortcuts (e.g. Shift+Arrow for ±6 hours) — separate feature.

## Decisions

### 1. Handle `KeyEventKind::Repeat` alongside `Press`

**Choice:** Accept both `Press` and `Repeat` events for left/right keys.

**Rationale:** Terminals emit `Repeat` events at the OS key-repeat rate when a key is held. This is the standard mechanism for detecting held keys in crossterm. Ignoring `Repeat` was the reason holding a key did nothing.

**Alternative considered:** Infer "held" from rapid consecutive `Press` events with timing heuristics. Rejected — fragile, platform-dependent, and `Repeat` already solves it.

### 2. Time-based acceleration ramp with local event-loop state

**Choice:** Track acceleration state as local variables in the event loop (`nav_start: Option<Instant>`, `nav_dir: i32`). Compute step size from elapsed time since the first press in the current direction:

| Elapsed since first press | Step size |
|---------------------------|-----------|
| < 400 ms                  | 1         |
| 400 ms – 1 s              | 2         |
| 1 s – 2 s                 | 4         |
| > 2 s                     | 8         |

**Rationale:** Time-based ramp feels natural — slow initially for precision, fast after holding. Local state keeps the `App` struct unchanged. The thresholds are tuned so the first ~6 presses at the OS repeat rate (~30 ms) stay at step=1, then ramp smoothly.

**Alternative considered:** Count-based (step increases every N events). Rejected — the repeat rate varies across OSes and user settings, so a count-based approach would feel inconsistent. Time is OS-agnostic.

### 3. Reset acceleration on direction change or non-arrow key

**Choice:** Reset `nav_start` to `None` when:
- A left/right event arrives in the opposite direction.
- Any non-left/right key is pressed.
- The poll times out with no pending offset (key was released).

**Rationale:** Reversing direction should restart from step=1 for precision. Other key presses (copy, quit) break the navigation flow. Timeout detection catches the key-release case since terminals don't send key-up events.

### 4. Step size applies to `pending_h_offset` accumulation

**Choice:** Instead of always adding ±1 to `pending_h_offset`, add ±step. The existing debounce and flush logic remains unchanged.

**Rationale:** Minimal change — the acceleration is a single multiplier on the existing ±1 logic. Rendering, offset application, and debounce all work as before.

## Risks / Trade-offs

- **[Feels too fast on high repeat-rate systems]** → The 8-hour cap and time-based ramp mitigate this. If it's still too fast, the thresholds are easy to adjust as constants.
- **[Repeat event availability]** → Some terminal emulators may not emit `Repeat` events. In that case, behavior degrades gracefully to the current single-step-per-press. No crash or incorrect behavior.
- **[Timeout-based release detection is approximate]** → A user who pauses briefly mid-hold may get their acceleration reset. Acceptable since re-ramping is quick (400 ms to step=2).
