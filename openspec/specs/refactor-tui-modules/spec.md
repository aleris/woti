### Requirement: TUI module structure preserves existing public API
The `tui` module SHALL continue to export `App` with public `new()` and `run()` methods after being restructured into submodules, so that callers (`main.rs`) require no changes.

#### Scenario: App is accessible via tui::App after refactor
- **WHEN** the `tui` module is restructured from a single file into a module directory
- **THEN** `tui::App::new(config)` and `app.run()` SHALL remain the public interface with identical signatures

### Requirement: Rendering output is identical after refactor
The visual output of the TUI SHALL remain pixel-identical after the module split — no changes to layout, colors, styles, or text content.

#### Scenario: Header renders identically
- **WHEN** the header rendering code is moved to a submodule
- **THEN** the header bar SHALL display the same title, date, time, and timezone abbreviation in the same style

#### Scenario: Timezone block renders identically
- **WHEN** the timezone block rendering is refactored into smaller functions within a submodule
- **THEN** each timezone row SHALL display the same info column, hour spans, am/pm spans, and day markers with identical styling

#### Scenario: Footer renders identically
- **WHEN** the footer rendering code is moved to a submodule
- **THEN** the shortcut bar and format switcher SHALL display the same keys, labels, and styles
