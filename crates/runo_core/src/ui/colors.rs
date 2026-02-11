use crate::Color;

pub type Rgb = (u8, u8, u8);

pub const APP_BG: Rgb = (18, 20, 23);
pub const PANEL_BG: Rgb = (29, 34, 41);
pub const PANEL_BG_ACTIVE: Rgb = (21, 49, 66);
pub const PANEL_BORDER: Rgb = (70, 80, 95);
pub const TEXT_PRIMARY: Rgb = (236, 241, 247);

pub fn rgb(rgb: Rgb) -> Color {
    Color::from_rgb8(rgb.0, rgb.1, rgb.2)
}
