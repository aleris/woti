## 1. Dependencies

- [x] 1.1 Add `arboard` crate to `Cargo.toml`

## 2. Copy Logic

- [x] 2.1 Add a method to `App` that builds the clipboard string: iterate all timezones, compute each one's selected hour, format as `City / Abbr Time` with optional day suffix when the day differs from the first timezone's day
- [x] 2.2 Handle 12h format (`4pm`) vs 24h format (`16:00`) based on `self.use_24h`
- [x] 2.3 Write the formatted string to the system clipboard using `arboard::Clipboard`

## 3. Key Handling

- [x] 3.1 Add `KeyCode::Char('c')` handler in the event loop that calls the copy method

## 4. Visual Feedback

- [x] 4.1 Add a `copied_at: Option<Instant>` field to `App` to track when the last copy occurred
- [x] 4.2 Update `render_footer` to show "Copied!" message when `copied_at` is within the last 2 seconds, otherwise show normal shortcut bar
- [x] 4.3 Add ` c ` / ` Copy ` shortcut to the footer shortcut bar

## 5. Error Handling

- [x] 5.1 Gracefully handle clipboard access failure (no crash, optionally show error in footer)
