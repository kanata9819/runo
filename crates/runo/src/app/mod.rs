pub(crate) mod events;
pub(crate) mod frame;
pub(crate) mod gpu;
pub(crate) mod gpu_runtime;
pub(crate) mod runner;
pub(crate) mod runtime;

use crate::ui::{EventBindings, Ui};
pub(crate) use runner::AppRunner;
pub use runtime::run;

#[cfg(test)]
#[path = "../../tests/unit/app/mod.rs"]
mod tests;

#[derive(Clone, Debug)]
pub struct RunOptions {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
    pub window_resizable: bool,
}

impl Default for RunOptions {
    fn default() -> Self {
        Self {
            window_title: "runo app".to_string(),
            window_width: 640,
            window_height: 480,
            window_resizable: true,
        }
    }
}

pub trait RunoApplication {
    type Event: 'static;

    fn build(&mut self, _ui: &mut Ui<'_>) {}
    fn event_bindings(&self) -> EventBindings<Self::Event>;
    fn on_event(&mut self, _ui: &mut Ui<'_>, _event: Self::Event) {}
    fn options(&self) -> RunOptions {
        RunOptions::default()
    }
}

pub(crate) fn build_runner<A: RunoApplication + 'static>(application: A) -> AppRunner<A> {
    let options = application.options();
    AppRunner::new(application, options)
}
