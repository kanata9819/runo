use crate::Color;
use crate::SliderResponse;
use crate::retained::UpsertSliderArgs;
use crate::ui::Ui;

pub(crate) struct ShowSliderArgs {
    pub(crate) id: String,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) min: f64,
    pub(crate) max: f64,
    pub(crate) value: Option<f64>,
    pub(crate) step: Option<f64>,
    pub(crate) text: Option<String>,
    pub(crate) font_size: f32,
    pub(crate) text_color: Color,
    pub(crate) enabled: bool,
}

impl<'a> Ui<'a> {
    pub(crate) fn show_slider(&mut self, args: ShowSliderArgs) -> SliderResponse {
        let ShowSliderArgs {
            id,
            width,
            height,
            min,
            max,
            value,
            step,
            text,
            font_size,
            text_color,
            enabled,
        } = args;
        let rect = self.allocate_widget_rect(width, height);
        self.retained.upsert_slider(UpsertSliderArgs {
            id,
            rect,
            min,
            max,
            value,
            step,
            text,
            font_size,
            text_color,
            enabled: self.resolve_enabled(enabled),
        })
    }
}
