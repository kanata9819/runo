use vello::peniko::Color;

#[derive(Clone, Debug)]
pub(crate) struct SliderCommon {
    pub(crate) min: f64,
    pub(crate) max: f64,
    pub(crate) value: Option<f64>,
    pub(crate) step: Option<f64>,
    pub(crate) text: Option<String>,
    pub(crate) font_size: f32,
    pub(crate) text_color: Color,
    pub(crate) enabled: bool,
}

impl Default for SliderCommon {
    fn default() -> Self {
        Self {
            min: 0.0,
            max: 100.0,
            value: None,
            step: None,
            text: None,
            font_size: 16.0,
            text_color: Color::from_rgb8(236, 241, 247),
            enabled: true,
        }
    }
}
