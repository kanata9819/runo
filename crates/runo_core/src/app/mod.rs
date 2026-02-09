pub(crate) mod events;
pub(crate) mod frame;
pub(crate) mod gpu;
pub(crate) mod runner;

use crate::ui::Ui;
pub(crate) use runner::AppRunner;

pub trait Application {
    fn build(&mut self, _ui: &mut Ui<'_>) {}
    fn update(&mut self, _ui: &mut Ui<'_>) {}
}

pub fn run<A: Application + 'static>(application: A) {
    let event_loop = winit::event_loop::EventLoop::new().expect("failed to create event loop");
    let mut app = AppRunner::new(application);
    event_loop.run_app(&mut app).expect("event loop failed");
}
