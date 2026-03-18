## ADDED Requirements

### Requirement: Scroll timeline with arrow keys
The user SHALL be able to press the left arrow key to shift the timeline one hour earlier and the right arrow key to shift it one hour later. The view re-renders to reflect the new offset.

#### Scenario: Press right arrow
- **WHEN** user presses the right arrow key once
- **THEN** the timeline shifts one hour to the right and re-renders showing the next hour set

#### Scenario: Press left arrow
- **WHEN** user presses the left arrow key once
- **THEN** the timeline shifts one hour to the left and re-renders showing the previous hour set

### Requirement: Debounced rendering during fast scrolling
When the user holds down an arrow key, the system SHALL debounce rendering to maintain responsiveness. Intermediate hour offsets MAY be skipped so that only the final position (or periodic snapshots) are rendered.

#### Scenario: Hold right arrow key
- **WHEN** user holds the right arrow key for 2 seconds
- **THEN** the timeline scrolls forward smoothly, skipping intermediate renders, and settles at the final offset when the key is released

#### Scenario: Single press remains instant
- **WHEN** user presses and releases the right arrow key once
- **THEN** the timeline shifts by exactly one hour without any debounce delay

### Requirement: Offset resets on re-launch
The timeline offset SHALL reset to the current hour each time the TUI is launched. It is not persisted.

#### Scenario: Relaunch after scrolling
- **WHEN** user scrolls to +5 hours, exits, and relaunches `woti`
- **THEN** the timeline starts centered on the current hour with zero offset

### Requirement: Current hour indicator follows offset
When the timeline is scrolled, the highlighted "current hour" column SHALL remain at the correct position relative to the displayed hours (i.e., it moves with the scroll so the actual current hour stays highlighted even when not centered).

#### Scenario: Current hour after scrolling
- **WHEN** user scrolls 3 hours to the right
- **THEN** the current hour column highlight moves 3 positions to the left within the visible strip, or scrolls off-screen if out of range
