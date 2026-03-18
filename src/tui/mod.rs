mod app;
mod copy;
mod event;
mod render;
mod theme;
mod time_format;

pub use app::App;

const CELL_WIDTH: u16 = 3;
const INFO_COL_WIDTH: u16 = 40;
const TIMELINE_GAP: u16 = 2;
const BLOCK_HEIGHT: u16 = 3;
const DEBOUNCE_MS: u64 = 50;
