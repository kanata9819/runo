use vello::Scene;
use vello::kurbo::{Affine, Rect};
use vello::peniko::{Fill, FontData};

use crate::ButtonResponse;
use crate::Color;
use crate::hooks::effect::{EffectCleanup, EffectStore};
use crate::layout::{LayoutDirection, LayoutNode};
use crate::retained::RetainedState;
use crate::widget::button::ButtonBuilder;
use crate::widget::label::LabelBuilder;

pub struct Ui<'a> {
    pub(crate) scene: &'a mut Scene,
    pub(crate) font: Option<FontData>,
    effects: &'a mut EffectStore,
    retained: &'a mut RetainedState,
    layout_stack: Vec<LayoutNode>,
    auto_id_counter: u64,
}

impl<'a> Ui<'a> {
    pub(crate) fn new(
        scene: &'a mut Scene,
        font: Option<FontData>,
        effects: &'a mut EffectStore,
        retained: &'a mut RetainedState,
    ) -> Self {
        Self {
            scene,
            font,
            effects,
            retained,
            layout_stack: vec![LayoutNode::new(
                (24.0, 24.0),
                LayoutDirection::Vertical,
                12.0,
            )],
            auto_id_counter: 0,
        }
    }

    pub fn button<'ui>(&'ui mut self) -> ButtonBuilder<'ui, 'a> {
        let id = format!("__auto_button_{}", self.auto_id_counter);
        self.auto_id_counter += 1;
        ButtonBuilder::new(self, id)
    }

    pub fn button_id<'ui>(&'ui mut self, id: impl Into<String>) -> ButtonBuilder<'ui, 'a> {
        ButtonBuilder::new(self, id.into())
    }

    pub fn label<'ui>(&'ui mut self, text: impl Into<String>) -> LabelBuilder<'ui, 'a> {
        let id = format!("__auto_label_{}", self.auto_id_counter);
        self.auto_id_counter += 1;
        LabelBuilder::new(self, id, text.into())
    }

    pub fn vertical<R>(&mut self, f: impl FnOnce(&mut Ui<'a>) -> R) -> R {
        self.with_layout(LayoutDirection::Vertical, 12.0, f)
    }

    pub fn horizontal<R>(&mut self, f: impl FnOnce(&mut Ui<'a>) -> R) -> R {
        self.with_layout(LayoutDirection::Horizontal, 12.0, f)
    }

    pub fn fill_rect(&mut self, x: f64, y: f64, w: f64, h: f64, color: Color) {
        let rect = Rect::new(x, y, x + w, y + h);
        self.scene
            .fill(Fill::NonZero, Affine::IDENTITY, color, None, &rect);
    }

    pub fn use_effect<D, F>(&mut self, id: impl Into<String>, deps: D, effect: F)
    where
        D: std::hash::Hash,
        F: FnOnce() -> Option<EffectCleanup>,
    {
        self.effects.use_effect(id, deps, effect);
    }

    pub fn button_state(&self, id: impl AsRef<str>) -> ButtonResponse {
        self.retained.button_response(id)
    }

    pub fn button_clicked(&self, id: impl AsRef<str>) -> bool {
        self.button_state(id).clicked
    }

    pub fn set_button_text(&mut self, id: impl AsRef<str>, text: impl Into<String>) {
        self.retained.set_button_text(id, Some(text.into()));
    }

    pub(crate) fn show_button(
        &mut self,
        id: String,
        width: f64,
        height: f64,
        text: Option<String>,
        text_color: Color,
    ) -> ButtonResponse {
        let (x, y) = self.allocate_rect(width, height);
        let rect = Rect::new(x, y, x + width, y + height);
        self.retained.upsert_button(id, rect, text, text_color)
    }

    pub(crate) fn show_label(
        &mut self,
        id: String,
        width: f64,
        height: f64,
        text: String,
        font_size: f32,
        text_color: Color,
    ) {
        let (x, y) = self.allocate_rect(width, height);
        let rect = Rect::new(x, y, x + width, y + height);
        self.retained
            .upsert_label(id, rect, text, font_size, text_color);
    }

    fn with_layout<R>(
        &mut self,
        direction: LayoutDirection,
        spacing: f64,
        f: impl FnOnce(&mut Ui<'a>) -> R,
    ) -> R {
        let origin = {
            let parent = self.layout_stack.last().expect("layout stack is empty");
            parent.place(0.0, 0.0)
        };
        self.layout_stack
            .push(LayoutNode::new(origin, direction, spacing));
        let result = f(self);
        let child = self.layout_stack.pop().expect("layout stack underflow");
        let (cw, ch) = child.consumed_size();
        self.advance_layout(cw, ch);
        result
    }

    pub(crate) fn allocate_rect(&mut self, width: f64, height: f64) -> (f64, f64) {
        let pos = {
            let layout = self.layout_stack.last().expect("layout stack is empty");
            layout.place(width, height)
        };
        self.advance_layout(width, height);
        pos
    }

    fn advance_layout(&mut self, width: f64, height: f64) {
        if let Some(layout) = self.layout_stack.last_mut() {
            layout.advance(width, height);
        }
    }
}
