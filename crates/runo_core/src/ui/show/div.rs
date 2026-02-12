use vello::kurbo::{Affine, Rect, RoundedRect, Stroke};
use vello::peniko::Fill;

use crate::Color;
use crate::layout::LayoutDirection;
use crate::ui::Ui;

pub(crate) struct ShowDivArgs {
    pub(crate) id: String,
    pub(crate) direction: LayoutDirection,
    pub(crate) gap: f64,
    pub(crate) width: Option<f64>,
    pub(crate) height: Option<f64>,
    pub(crate) padding_left: f64,
    pub(crate) padding_top: f64,
    pub(crate) padding_right: f64,
    pub(crate) padding_bottom: f64,
    pub(crate) bg_color: Option<Color>,
    pub(crate) border_color: Option<Color>,
    pub(crate) border_width: f64,
    pub(crate) radius: f64,
}

impl<'a> Ui<'a> {
    pub(crate) fn show_div<R>(&mut self, args: ShowDivArgs, f: impl FnOnce(&mut Ui<'a>) -> R) -> R {
        let ShowDivArgs {
            id,
            direction,
            gap,
            width,
            height,
            padding_left,
            padding_top,
            padding_right,
            padding_bottom,
            bg_color,
            border_color,
            border_width,
            radius,
        } = args;

        let origin = self.layout_stack.peek_next_position();
        let (div_visible, effective_enabled, bg_color) = self.resolve_div_style(&id, bg_color);
        let (result, content_w, content_h) = self.layout_div_children(
            origin,
            (padding_left, padding_top),
            direction,
            gap,
            effective_enabled,
            f,
        );

        let auto_w = content_w + padding_left + padding_right;
        let auto_h = content_h + padding_top + padding_bottom;
        let div_w = width.unwrap_or(auto_w);
        let div_h = height.unwrap_or(auto_h);

        self.paint_div(
            origin,
            (div_w, div_h),
            radius,
            div_visible,
            bg_color,
            border_color,
            border_width,
        );

        self.layout_stack.advance_current(div_w, div_h);
        result
    }

    fn resolve_div_style(
        &self,
        id: &str,
        default_bg_color: Option<Color>,
    ) -> (bool, bool, Option<Color>) {
        let div_visible = self.retained.div_visible(id);
        let div_enabled = self.retained.div_enabled(id);
        let effective_enabled = self.current_enabled() && div_enabled;
        let bg_color = self.retained.div_background(id).or(default_bg_color);
        (div_visible, effective_enabled, bg_color)
    }

    fn layout_div_children<R>(
        &mut self,
        origin: (f64, f64),
        padding: (f64, f64),
        direction: LayoutDirection,
        gap: f64,
        effective_enabled: bool,
        f: impl FnOnce(&mut Ui<'a>) -> R,
    ) -> (R, f64, f64) {
        let content_origin = (origin.0 + padding.0, origin.1 + padding.1);
        self.layout_stack
            .push_layout_at(content_origin, direction, gap);
        self.enabled_stack.push(effective_enabled);
        let result = f(self);
        let _ = self.enabled_stack.pop();
        let (content_w, content_h) = self.layout_stack.pop_layout_consumed();
        (result, content_w, content_h)
    }

    fn paint_div(
        &mut self,
        origin: (f64, f64),
        size: (f64, f64),
        radius: f64,
        div_visible: bool,
        bg_color: Option<Color>,
        border_color: Option<Color>,
        border_width: f64,
    ) {
        let rect = Rect::new(origin.0, origin.1, origin.0 + size.0, origin.1 + size.1);
        let rounded = RoundedRect::from_rect(rect, radius);

        if div_visible && let Some(color) = bg_color {
            self.scene
                .fill(Fill::NonZero, Affine::IDENTITY, color, None, &rounded);
        }

        if div_visible && let Some(color) = border_color {
            self.scene.stroke(
                &Stroke::new(border_width.max(0.0)),
                Affine::IDENTITY,
                color,
                None,
                &rounded,
            );
        }
    }
}
