use crate::app::{AppRunner, RunoApplication, build_runner};

pub fn run<A: RunoApplication + 'static>(application: A) {
    let event_loop = winit::event_loop::EventLoop::new().expect("failed to create event loop");
    let mut app: AppRunner<A> = build_runner(application);

    event_loop.run_app(&mut app).expect("event loop failed");
}
