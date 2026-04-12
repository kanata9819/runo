use vello::peniko::Color;

#[derive(Clone, Debug)]
pub(crate) struct CheckboxCommon {
    pub(crate) text: Option<String>,
    pub(crate) checked: Option<bool>,
    pub(crate) font_size: f32,
    pub(crate) text_color: Color,
    pub(crate) enabled: bool,
}

impl Default for CheckboxCommon {
    fn default() -> Self {
        Self {
            text: None,
            checked: None,
            font_size: 18.0,
            text_color: Color::from_rgb8(236, 241, 247),
            enabled: true,
        }
    }
}
