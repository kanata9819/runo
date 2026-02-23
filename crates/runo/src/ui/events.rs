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

#[cfg(test)]
mod tests {
    use vello::Scene;

    use crate::hooks::effect::EffectStore;
    use crate::hooks::state::StateStore;
    use crate::retained::RetainedState;
    use crate::ui::Ui;

    #[test]
    fn ui_events_empty_queue_returns_none_and_empty_vec() {
        let mut scene = Scene::new();
        let mut effects = EffectStore::new();
        let mut states = StateStore::new();
        let mut retained = RetainedState::new();
        let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

        assert!(ui.events().next_event().is_none());
        assert!(ui.events().drain_events().is_empty());
    }
}
