# Design: Add --reset flag to remove command

## Approach

Use clap's `conflicts_with` attribute to make `--reset` mutually exclusive with the positional `zone` argument. When `--reset` is provided, skip timezone resolution entirely and replace the config with `AppConfig::default()`.

## Changes

### cli.rs

Add a `reset` field to the `Remove` variant:

```rust
Remove {
    #[arg(long, conflicts_with = "zone")]
    reset: bool,

    #[arg(required_unless_present = "reset")]
    zone: Vec<String>,
}
```

`conflicts_with` ensures `woti remove --reset PST` is rejected by clap. `required_unless_present` ensures bare `woti remove` (no zone, no flag) still shows an error.

### main.rs

Branch in the dispatch:

```rust
Some(Command::Remove { zone, reset }) => {
    if reset {
        cmd_reset();
    } else {
        cmd_remove(&zone.join(" "));
    }
}
```

New `cmd_reset()` function:
- Create a fresh `AppConfig::default()`
- Save it to disk
- Print confirmation with the count of removed timezones

### config.rs

Add `AppConfig::reset()`:

```rust
pub fn reset(&mut self) -> usize {
    let removed = self.timezones.iter().filter(|e| !e.is_default).count();
    self.timezones.retain(|e| e.is_default);
    removed
}
```

Returns the count of removed entries for the confirmation message.

## Risks

None significant. The flag is additive and the mutual exclusion is enforced at the CLI layer by clap.
