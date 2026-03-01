use crate::layout::{LayoutDirection, LayoutNode};

pub(crate) struct LayoutStack {
    stack: Vec<LayoutNode>,
}

impl LayoutStack {
    pub(crate) fn new(origin: (f64, f64), direction: LayoutDirection, spacing: f64) -> Self {
        Self {
            stack: vec![LayoutNode::new(origin, direction, spacing)],
        }
    }

    pub(crate) fn push_layout(&mut self, direction: LayoutDirection, spacing: f64) {
        let origin = self.peek_next_position();
        self.push_layout_at(origin, direction, spacing);
    }

    pub(crate) fn push_layout_at(
        &mut self,
        origin: (f64, f64),
        direction: LayoutDirection,
        spacing: f64,
    ) {
        self.stack.push(LayoutNode::new(origin, direction, spacing));
    }

    pub(crate) fn pop_layout_and_advance_parent(&mut self) {
        let (cw, ch) = self.pop_layout_consumed();
        self.advance_current(cw, ch);
    }

    pub(crate) fn pop_layout_consumed(&mut self) -> (f64, f64) {
        let child = self.stack.pop().expect("layout stack underflow");
        child.consumed_size()
    }

    pub(crate) fn allocate_rect(&mut self, width: f64, height: f64) -> (f64, f64) {
        let pos = {
            let layout = self.stack.last().expect("layout stack is empty");
            layout.place(width, height)
        };
        self.advance_current(width, height);
        pos
    }

    pub(crate) fn peek_next_position(&self) -> (f64, f64) {
        let layout = self.stack.last().expect("layout stack is empty");
        layout.place(0.0, 0.0)
    }

    pub(crate) fn advance_current(&mut self, width: f64, height: f64) {
        if let Some(layout) = self.stack.last_mut() {
            layout.advance(width, height);
        }
    }
}

#[cfg(test)]
#[path = "../../tests/unit/layout/stack.rs"]
mod tests;
