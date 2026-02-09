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
        let origin = {
            let parent = self.stack.last().expect("layout stack is empty");
            parent.place(0.0, 0.0)
        };
        self.stack.push(LayoutNode::new(origin, direction, spacing));
    }

    pub(crate) fn pop_layout_and_advance_parent(&mut self) {
        let child = self.stack.pop().expect("layout stack underflow");
        let (cw, ch) = child.consumed_size();
        self.advance(cw, ch);
    }

    pub(crate) fn allocate_rect(&mut self, width: f64, height: f64) -> (f64, f64) {
        let pos = {
            let layout = self.stack.last().expect("layout stack is empty");
            layout.place(width, height)
        };
        self.advance(width, height);
        pos
    }

    fn advance(&mut self, width: f64, height: f64) {
        if let Some(layout) = self.stack.last_mut() {
            layout.advance(width, height);
        }
    }
}
