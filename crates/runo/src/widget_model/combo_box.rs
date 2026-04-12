use vello::peniko::Color;

#[derive(Clone, Debug)]
pub(crate) struct ComboBoxCommon {
    pub(crate) items: Vec<String>,
    pub(crate) selected_index: Option<usize>,
    pub(crate) font_size: f32,
    pub(crate) text_color: Color,
    pub(crate) bg_color: Color,
    pub(crate) border_color: Color,
    pub(crate) enabled: bool,
}

impl Default for ComboBoxCommon {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            selected_index: None,
            font_size: 18.0,
            text_color: Color::from_rgb8(236, 241, 247),
            bg_color: Color::from_rgb8(33, 38, 46),
            border_color: Color::from_rgb8(78, 89, 104),
            enabled: true,
        }
    }
}
