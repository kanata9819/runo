#[derive(Clone, Copy)]
pub(crate) enum LayoutDirection {
    Vertical,
    Horizontal,
}

pub(crate) struct LayoutNode {
    origin: (f64, f64),
    cursor: f64,
    cross: f64,
    direction: LayoutDirection,
    spacing: f64,
}

impl LayoutNode {
    pub(crate) fn new(origin: (f64, f64), direction: LayoutDirection, spacing: f64) -> Self {
        Self {
            origin,
            cursor: 0.0,
            cross: 0.0,
            direction,
            spacing,
        }
    }

    pub(crate) fn place(&self, _width: f64, _height: f64) -> (f64, f64) {
        match self.direction {
            LayoutDirection::Vertical => (self.origin.0, self.origin.1 + self.cursor),
            LayoutDirection::Horizontal => (self.origin.0 + self.cursor, self.origin.1),
        }
    }

    pub(crate) fn advance(&mut self, width: f64, height: f64) {
        match self.direction {
            LayoutDirection::Vertical => {
                self.cursor += height + self.spacing;
                self.cross = self.cross.max(width);
            }
            LayoutDirection::Horizontal => {
                self.cursor += width + self.spacing;
                self.cross = self.cross.max(height);
            }
        }
    }

    pub(crate) fn consumed_size(&self) -> (f64, f64) {
        let main = if self.cursor > 0.0 {
            self.cursor - self.spacing
        } else {
            0.0
        };
        match self.direction {
            LayoutDirection::Vertical => (self.cross, main),
            LayoutDirection::Horizontal => (main, self.cross),
        }
    }
}
