use vello::peniko::Color;

use crate::widget::text_box::Overflow;

#[derive(Clone, Debug)]
pub(crate) struct TextBoxCommon {
    pub(crate) text: Option<String>,
    pub(crate) placeholder: Option<String>,
    pub(crate) font_size: f32,
    pub(crate) text_color: Color,
    pub(crate) bg_color: Color,
    pub(crate) border_color: Color,
    pub(crate) disable_border: bool,
    pub(crate) enabled: bool,
    pub(crate) read_only: bool,
    pub(crate) overflow_x: Overflow,
    pub(crate) overflow_y: Overflow,
}

impl Default for TextBoxCommon {
    fn default() -> Self {
        Self {
            text: None,
            placeholder: None,
            font_size: 18.0,
            text_color: Color::from_rgb8(236, 241, 247),
            bg_color: Color::from_rgb8(33, 38, 46),
            border_color: Color::from_rgb8(78, 89, 104),
            disable_border: false,
            enabled: true,
            read_only: false,
            overflow_x: Overflow::Auto,
            overflow_y: Overflow::Hidden,
        }
    }
}
