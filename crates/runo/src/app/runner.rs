use std::num::NonZero;
use std::sync::Arc;

use vello::Scene;
use vello::peniko::FontData;
use vello::util::{RenderContext, RenderSurface};
use vello::wgpu;
use vello::{AaSupport, Renderer, RendererOptions};
use winit::dpi::LogicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes, WindowId};

use crate::app::{RunOptions, RunoApplication};
use crate::font::load_default_font;
use crate::hooks::use_effect::EffectStore;
use crate::hooks::use_state::StateStore;
use crate::input::InputState;
use crate::retained::RetainedState;

#[cfg(test)]
#[path = "../../tests/unit/app/runner.rs"]
mod tests;

fn sanitize_window_size(width: u32, height: u32) -> (u32, u32) {
    (width.max(1), height.max(1))
}

fn window_attributes_from_options(options: &RunOptions) -> WindowAttributes {
    Window::default_attributes()
        .with_title(options.window_title.clone())
        .with_inner_size(LogicalSize::new(
            options.window_width as f64,
            options.window_height as f64,
        ))
        .with_resizable(options.window_resizable)
}

pub(crate) struct AppRunner<A: RunoApplication + 'static> {
    pub(super) user_app: A,
    pub(super) window: Option<Arc<Window>>,
    pub(super) window_id: Option<WindowId>,
    pub(super) render_cx: RenderContext,
    pub(super) surface: Option<RenderSurface<'static>>,
    pub(super) renderer: Option<Renderer>,
    pub(super) scene: Scene,
    pub(super) input: InputState,
    pub(super) font: Option<FontData>,
    pub(super) effects: EffectStore,
    pub(super) states: StateStore,
    pub(super) retained: RetainedState,
    window_options: RunOptions,
}

impl<A: RunoApplication + 'static> AppRunner<A> {
    pub(super) fn new(user_app: A, mut window_options: RunOptions) -> Self {
        (window_options.window_width, window_options.window_height) =
            sanitize_window_size(window_options.window_width, window_options.window_height);
        Self {
            user_app,
            window: None,
            window_id: None,
            render_cx: RenderContext::new(),
            surface: None,
            renderer: None,
            scene: Scene::new(),
            input: InputState::default(),
            font: load_default_font(),
            effects: EffectStore::new(),
            states: StateStore::new(),
            retained: RetainedState::new(),
            window_options,
        }
    }

    pub(super) fn init_window_and_gpu(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = window_attributes_from_options(&self.window_options);

        let window = Arc::new(
            event_loop
                .create_window(attributes)
                .expect("failed to create window"),
        );
        window.set_ime_allowed(true);
        let size = window.inner_size();

        let surface = pollster::block_on(self.render_cx.create_surface(
            window.clone(),
            size.width,
            size.height,
            wgpu::PresentMode::AutoVsync,
        ))
        .expect("failed to create surface");

        let device = &self.render_cx.devices[surface.dev_id].device;
        let renderer = Renderer::new(
            device,
            RendererOptions {
                pipeline_cache: None,
                use_cpu: false,
                antialiasing_support: AaSupport::all(),
                num_init_threads: Some(NonZero::new(1).expect("non-zero")),
            },
        )
        .expect("renderer init failed");

        self.window_id = Some(window.id());
        self.window = Some(window);
        self.surface = Some(surface);
        self.renderer = Some(renderer);
    }

    pub(super) fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        if let Some(surface) = self.surface.as_mut() {
            self.render_cx.resize_surface(surface, width, height);
        }
    }

    pub(super) fn request_redraw(&self) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    #[cfg(test)]
    #[cfg(test)]
    pub(super) fn window_options(&self) -> &RunOptions {
        &self.window_options
    }
}
