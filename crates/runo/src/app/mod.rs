pub(crate) mod events;
pub(crate) mod frame;
pub(crate) mod gpu;
pub(crate) mod runner;

use crate::ui::Ui;
pub(crate) use runner::AppRunner;

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
    fn build(&mut self, _ui: &mut Ui<'_>) {}
    fn update(&mut self, _ui: &mut Ui<'_>) {}
    fn options(&self) -> RunOptions {
        RunOptions::default()
    }
}

pub fn run<A: RunoApplication + 'static>(application: A) {
    let options = application.options();
    let event_loop = winit::event_loop::EventLoop::new().expect("failed to create event loop");
    let mut app = AppRunner::new(application, options);

    event_loop.run_app(&mut app).expect("event loop failed");
}

#[cfg(test)]
mod tests {
    use super::*;

    struct App;
    impl RunoApplication for App {}

    #[test]
    fn run_options_default_values() {
        let options = RunOptions::default();
        assert_eq!(options.window_title, "runo app");
        assert_eq!(options.window_width, 640);
        assert_eq!(options.window_height, 480);
    }

    #[test]
    fn application_default_options_delegate_to_run_options_default() {
        let app = App;
        let options = app.options();
        assert_eq!(options.window_title, "runo app");
        assert_eq!(options.window_width, 640);
        assert_eq!(options.window_height, 480);
    }
}
