use vello::peniko::Color;

#[derive(Clone, Debug)]
pub(crate) struct RadioButtonCommon {
    pub(crate) group: String,
    pub(crate) text: Option<String>,
    pub(crate) selected: Option<bool>,
    pub(crate) font_size: f32,
    pub(crate) text_color: Color,
    pub(crate) enabled: bool,
}

impl Default for RadioButtonCommon {
    fn default() -> Self {
        Self {
            group: "default".to_string(),
            text: None,
            selected: None,
            font_size: 18.0,
            text_color: Color::from_rgb8(236, 241, 247),
            enabled: true,
        }
    }
}
