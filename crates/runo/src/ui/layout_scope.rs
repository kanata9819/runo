use vello::kurbo::Rect;

use super::Ui;
use crate::layout::LayoutDirection;

impl Ui<'_> {
    pub fn vertical<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        self.with_layout(LayoutDirection::Vertical, 12.0, f)
    }

    pub fn horizontal<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        self.with_layout(LayoutDirection::Horizontal, 12.0, f)
    }

    pub fn with_stable_key<R, K>(&mut self, key: K, f: impl FnOnce(&mut Self) -> R) -> R
    where
        K: Into<String>,
    {
        self.key_scope_stack.push(key.into());
        self.auto_id_counter_stack.push(0);
        let result: R = f(self);
        let _popped_auto_id_counter: Option<u64> = self.auto_id_counter_stack.pop();
        let _popped_key_scope: Option<String> = self.key_scope_stack.pop();

        result
    }

    fn with_layout<R>(
        &mut self,
        direction: LayoutDirection,
        spacing: f64,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        self.layout_stack.push_layout(direction, spacing);
        let result: R = f(self);
        self.layout_stack.pop_layout_and_advance_parent();
        result
    }

    pub(crate) fn allocate_rect(&mut self, width: f64, height: f64) -> (f64, f64) {
        self.layout_stack.allocate_rect(width, height)
    }

    pub(crate) fn allocate_widget_rect(&mut self, width: f64, height: f64) -> Rect {
        let (x, y): (f64, f64) = self.allocate_rect(width, height);
        Rect::new(x, y, x + width, y + height)
    }

    fn current_enabled(&self) -> bool {
        self.enabled_stack.last().copied().unwrap_or(true)
    }

    pub(crate) fn resolve_enabled(&self, enabled: bool) -> bool {
        enabled && self.current_enabled()
    }
}
