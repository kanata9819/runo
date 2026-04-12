use vello::kurbo::Rect;

use crate::SliderResponse;
use crate::retained::UpsertSliderArgs;
use crate::ui::Ui;
use crate::widget_model::slider::SliderCommon;

pub(crate) struct ShowSliderArgs {
    pub(crate) id: String,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) common: SliderCommon,
}

impl Ui<'_> {
    pub(crate) fn show_slider(&mut self, args: ShowSliderArgs) -> SliderResponse {
        let ShowSliderArgs {
            id,
            width,
            height,
            mut common,
        } = args;
        let rect: Rect = self.allocate_widget_rect(width, height);
        common.enabled = self.resolve_enabled(common.enabled);
        let SliderCommon {
            min,
            max,
            value,
            step,
            text,
            font_size,
            text_color,
            enabled,
        } = common;
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
            enabled,
        })
    }
}
