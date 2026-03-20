use ratatui::style::Color;

// -- Header --
pub const HEADER_BG: Color = Color::Rgb(25, 30, 45);
pub const HEADER_ICON: Color = Color::Yellow;
pub const HEADER_TITLE: Color = Color::Rgb(255, 127, 80);


// -- Footer --
pub const FOOTER_BG: Color = Color::Rgb(38, 38, 42);
pub const COPIED: Color = Color::Green;

pub const KEY_FG: Color = Color::Black;
pub const KEY_BG: Color = Color::Gray;
pub const KEY_DISABLED_FG: Color = Color::Rgb(90, 90, 90);
pub const KEY_DISABLED_BG: Color = Color::Rgb(50, 50, 50);
pub const LABEL_FG: Color = Color::Gray;
pub const LABEL_DISABLED_FG: Color = Color::Rgb(75, 75, 75);

pub const SWITCHER_ACTIVE_FG: Color = Color::Cyan;
pub const SWITCHER_ACTIVE_BG: Color = Color::Rgb(60, 60, 60);
pub const SWITCHER_DIM_FG: Color = Color::Gray;
pub const SWITCHER_SEP: Color = Color::Rgb(80, 80, 80);

// -- Timeline --
pub const SELECTED_FG: Color = Color::Black;
pub const SELECTED_BG: Color = Color::Rgb(255, 255, 224);
pub const LOCAL_BG: Color = Color::Rgb(50, 50, 50);
pub const HOUR_FG: Color = Color::White;
pub const HOUR_FG_TRANSITION: Color = Color::Rgb(140, 140, 140);
pub const HOUR_FG_NIGHT: Color = Color::Rgb(75, 75, 75);
pub const AMPM_FG: Color = Color::Gray;
pub const AMPM_FG_TRANSITION: Color = Color::Rgb(120, 120, 120);
pub const AMPM_FG_NIGHT: Color = Color::Rgb(60, 60, 60);
pub const DAY_LABEL: Color = Color::DarkGray;

// -- Timezone info --
pub const CITY_FG: Color = Color::White;
pub const TZ_BADGE_FG: Color = Color::White;
pub const TZ_BADGE_BG: Color = Color::DarkGray;
pub const OFFSET_FG: Color = Color::Cyan;
pub const TIME_FG: Color = Color::Blue;
pub const SECONDARY_FG: Color = Color::DarkGray;
