use super::Ui;
use crate::event::UiEvent;

pub struct UiEvents<'ui, 'a> {
    pub(super) ui: &'ui mut Ui<'a>,
}

impl<'ui, 'a> UiEvents<'ui, 'a> {
    pub fn next_event(&mut self) -> Option<UiEvent> {
        self.ui.retained.pop_event()
    }

    pub fn drain_events(&mut self) -> Vec<UiEvent> {
        self.ui.retained.drain_events()
    }
}
