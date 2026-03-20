## ADDED Requirements

### Requirement: Hour labels use DST-aware wall-clock time

The hour digit displayed in each timeline cell SHALL be derived from
`compute_datetime_for_hour(tz, now_tz, h - current_hour).hour()` rather than arithmetic modular reduction.

#### Scenario: Spring-forward transition (hour skipped)

- **WHEN** the user is in America/New_York on the night of spring-forward (March second Sunday) and scrolls the timeline
  past 2:00 AM EST
- **THEN** the timeline hour labels SHALL jump from 1 to 3 (skipping 2), matching the wall-clock transition to EDT

#### Scenario: Fall-back transition (hour repeated)

- **WHEN** the user is in America/New_York on the night of fall-back (November first Sunday) and scrolls the timeline
  past 1:00 AM EDT
- **THEN** the timeline hour labels SHALL display the hour returned by `compute_datetime_for_hour` for each cell,
  consistent with chrono's resolution of the ambiguous time

#### Scenario: No DST zone is unaffected

- **WHEN** the user is in a timezone without DST (e.g., UTC, Asia/Kolkata)
- **THEN** the hour labels SHALL be identical to the previous arithmetic-based rendering

### Requirement: AM/PM labels use DST-aware wall-clock time

The AM/PM indicator (or 24h minute superscript) and its working-hours shade color SHALL be derived from the same
DST-aware hour used for the hour digit.

#### Scenario: Spring-forward AM/PM consistency

- **WHEN** the timeline skips hour 2 during spring-forward
- **THEN** the AM/PM label for the cell that would have shown "2 am" SHALL instead show "3 am"

#### Scenario: AM/PM shading matches wall-clock hour

- **WHEN** working-hours shading is enabled and a DST transition falls within the visible timeline
- **THEN** the shade color for each cell SHALL correspond to the DST-aware wall-clock hour, not the arithmetic hour

### Requirement: Day boundary detection uses DST-aware wall-clock time

The midnight boundary check that triggers a day-label insertion SHALL use `compute_datetime_for_hour(...).hour() == 0`
instead of `((h % 24) + 24) % 24 == 0`.

#### Scenario: Day label placement at DST spring-forward

- **WHEN** DST spring-forward occurs at midnight (hypothetical zone) or the arithmetic hour wraps to 0 but the
  wall-clock hour does not
- **THEN** the day label SHALL appear only when the resolved wall-clock hour is truly 0

#### Scenario: Day label text is already DST-aware

- **WHEN** a day label is placed at a midnight boundary
- **THEN** the date text (weekday, day number) SHALL continue to use `compute_datetime_for_hour` as it does today — this
  requirement confirms no regression

### Requirement: Displayed hours match copied text

The hour shown in the timeline for any cell SHALL match the hour produced by `copy.rs` for the same hour offset.

#### Scenario: Copy after scrolling across DST boundary

- **WHEN** the user scrolls across a DST boundary and copies the selection
- **THEN** the hour in the copied text SHALL be identical to the hour displayed in the timeline for the selected cell
