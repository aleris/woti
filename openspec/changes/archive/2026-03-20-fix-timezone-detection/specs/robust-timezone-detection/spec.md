## ADDED Requirements

### Requirement: TZ environment variable takes priority
The system SHALL check the `TZ` environment variable first when detecting the local timezone. If `TZ` is set, non-empty, and contains a valid IANA timezone name, it SHALL be used as the detected timezone.

#### Scenario: TZ set to a valid IANA timezone
- **WHEN** the `TZ` environment variable is set to `America/New_York`
- **THEN** `localtime_iana()` SHALL return `Some("America/New_York")`

#### Scenario: TZ set to an invalid value
- **WHEN** the `TZ` environment variable is set to `EST5EDT` (a POSIX-style string, not IANA)
- **THEN** the system SHALL skip the `TZ` value and proceed to the next detection layer

#### Scenario: TZ is empty
- **WHEN** the `TZ` environment variable is set to an empty string
- **THEN** the system SHALL skip the `TZ` value and proceed to the next detection layer

#### Scenario: TZ is not set
- **WHEN** the `TZ` environment variable is not set in the environment
- **THEN** the system SHALL skip to the next detection layer

### Requirement: Cross-platform detection via iana-time-zone crate
The system SHALL use the `iana-time-zone` crate as the second detection layer, after the `TZ` environment variable. This provides native timezone detection on macOS, Linux, and Windows.

#### Scenario: Platform-native detection succeeds
- **WHEN** the `TZ` variable is not set and the `iana-time-zone` crate successfully detects the system timezone
- **THEN** `localtime_iana()` SHALL return the timezone name reported by the crate

#### Scenario: Platform-native detection fails
- **WHEN** the `iana-time-zone` crate returns an error
- **THEN** the system SHALL fall through to the Unix filesystem heuristic (on Unix) or return `None` (on non-Unix)

### Requirement: Improved Unix filesystem fallback
On Unix systems, the system SHALL attempt to resolve `/etc/localtime` via `std::fs::canonicalize` (instead of `read_link`) and extract the IANA timezone name from the canonical path by splitting on `/zoneinfo/`. This works for both symlinks and regular file copies.

#### Scenario: /etc/localtime is a symlink
- **WHEN** `/etc/localtime` is a symlink to `/usr/share/zoneinfo/Europe/Berlin`
- **THEN** `localtime_iana()` SHALL return `Some("Europe/Berlin")`

#### Scenario: /etc/localtime is a regular file in the zoneinfo tree
- **WHEN** `/etc/localtime` is a regular file whose canonical path resolves to `/usr/share/zoneinfo/Asia/Tokyo`
- **THEN** `localtime_iana()` SHALL return `Some("Asia/Tokyo")`

#### Scenario: /etc/localtime does not contain a zoneinfo path
- **WHEN** `/etc/localtime` exists but its canonical path does not contain `/zoneinfo/`
- **THEN** the system SHALL return `None`

### Requirement: Detection priority chain
The system SHALL attempt timezone detection in this order, returning the first successful result:
1. `TZ` environment variable
2. `iana-time-zone` crate
3. Unix filesystem heuristic (`/etc/localtime` canonicalization) — Unix only
4. `None`

#### Scenario: All layers attempted in order
- **WHEN** `TZ` is not set, `iana-time-zone` fails, and `/etc/localtime` does not exist
- **THEN** `localtime_iana()` SHALL return `None`

#### Scenario: Earlier layer succeeds, later layers skipped
- **WHEN** `TZ` is set to `Pacific/Auckland`
- **THEN** the `iana-time-zone` crate and filesystem heuristic SHALL NOT be invoked
