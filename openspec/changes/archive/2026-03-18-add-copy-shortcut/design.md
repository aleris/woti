## Context

woti is a Rust TUI app built with ratatui/crossterm that displays a horizontal timeline of hours across multiple configured timezones. Users navigate left/right to shift the selected hour column. The selected column is highlighted in yellow across all timezone rows.

Currently there is no way to extract data from the TUI — users must manually transcribe timezone information when sharing meeting times.

## Goals / Non-Goals

**Goals:**
- Let users copy the selected hour for all timezones to the system clipboard with a single keypress (`c`)
- Produce a clean, human-readable multi-line format suitable for pasting into chat/email
- Include day/date only when a timezone's selected hour falls on a different calendar day than the first timezone

**Non-Goals:**
- Copying arbitrary ranges of hours or multiple columns
- Customizable output format or templates
- Copy-to-file or export functionality

## Decisions

### 1. Clipboard crate: `arboard`

Use the `arboard` crate for clipboard access.

**Rationale**: `arboard` is the most actively maintained cross-platform clipboard crate for Rust. It supports macOS, Linux (X11/Wayland), and Windows without requiring external tools. It works without a display server in some configurations. Alternative `cli-clipboard` shells out to `pbcopy`/`xclip` which adds external dependencies.

### 2. Output format

Each line: `City / Abbreviation Hour[am/pm]` with optional day suffix.

```
Bucharest / EET 4pm
San Jose / PDT 7pm
```

When a timezone's hour falls on a different calendar day than the first timezone in the list:

```
Bucharest / EET 4pm
San Jose / PDT 8pm WED 19
```

**Rationale**: Compact, readable, matches what you'd type in a chat message. The day suffix only appears when needed to avoid noise. The first timezone serves as the reference day — it never shows a day suffix itself.

### 3. Time format follows the app's 12h/24h setting

If the app is in 24h mode, copy `16:00` instead of `4pm`. This keeps the copied text consistent with what the user sees on screen.

### 4. Visual feedback via transient footer message

On successful copy, briefly replace the footer content with a "Copied!" confirmation. Use a timer (2 seconds) to revert to the normal shortcut bar. This avoids adding a separate toast/notification system.

### 5. Build the copy string from existing data in `App`

The copy logic reuses the same timezone iteration and hour computation already in `render_timezone_block`. Extract a method that computes the selected hour's display data for a given timezone entry, then format it for clipboard output.

## Risks / Trade-offs

- **[Clipboard access in terminal]** → Some headless or SSH environments may not have clipboard access. `arboard` will return an error; we silently skip the copy (no crash) and could show an error hint in the footer.
- **[New dependency]** → `arboard` adds a compile-time dependency. Acceptable since clipboard is fundamental to the feature and there's no stdlib alternative.
- **[Footer flash timing]** → The 2-second confirmation relies on the existing render loop's tick interval. Since woti already re-renders on a 250ms poll, the timing will be approximate but good enough.
