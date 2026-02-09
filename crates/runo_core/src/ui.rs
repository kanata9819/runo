use vello::kurbo::{Affine, Rect};
use vello::peniko::{Fill, FontData};
use vello::Scene;

use crate::hooks::effect::{EffectCleanup, EffectStore};
use crate::input::InputFrame;
use crate::layout::{LayoutDirection, LayoutNode};
use crate::widget::button::ButtonBuilder;
use crate::Color;

pub struct Ui<'a> {
    pub(crate) scene: &'a mut Scene,
    pub(crate) input: InputFrame,
    pub(crate) active_button: &'a mut Option<u64>,
    pub(crate) font: Option<FontData>,
    effects: &'a mut EffectStore,
    layout_stack: Vec<LayoutNode>,
    auto_id_counter: u64,
}

impl<'a> Ui<'a> {
    pub(crate) fn new(
        scene: &'a mut Scene,
        input: InputFrame,
        active_button: &'a mut Option<u64>,
        font: Option<FontData>,
        effects: &'a mut EffectStore,
    ) -> Self {
        Self {
            scene,
            input,
            active_button,
            font,
            effects,
            layout_stack: vec![LayoutNode::new((24.0, 24.0), LayoutDirection::Vertical, 12.0)],
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
