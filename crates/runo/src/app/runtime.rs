use crate::app::{self, AppRunner, RunoApplication};

pub fn run<A, Event>(application: A)
where
    A: RunoApplication<Event> + 'static,
    Event: 'static,
{
    let event_loop: winit::event_loop::EventLoop<()> =
        winit::event_loop::EventLoop::new().expect("failed to create event loop");
    let mut app: AppRunner<A, Event> = app::build_runner(application);
    event_loop.run_app(&mut app).expect("event loop failed");
}
