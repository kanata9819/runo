use std::num::NonZero;
use std::sync::Arc;

use vello::kurbo::{Affine, Rect};
use vello::peniko::{Fill, FontData};
use vello::util::{RenderContext, RenderSurface};
use vello::wgpu;
use vello::{AaConfig, AaSupport, RenderParams, Renderer, RendererOptions, Scene};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

use crate::font::load_default_font;
use crate::hooks::effect::EffectStore;
use crate::input::InputState;
use crate::ui::Ui;
use crate::Color;

pub trait Application {
    fn update(&mut self, ui: &mut Ui<'_>);
}

pub fn run<A: Application + 'static>(application: A) {
    let event_loop = EventLoop::new().expect("failed to create event loop");
    let mut app = AppRunner::new(application);
    event_loop.run_app(&mut app).expect("event loop failed");
}

struct AppRunner<A: Application + 'static> {
    user_app: A,
    window: Option<Arc<Window>>,
    window_id: Option<WindowId>,
    render_cx: RenderContext,
    surface: Option<RenderSurface<'static>>,
    renderer: Option<Renderer>,
    scene: Scene,
    input: InputState,
    active_button: Option<u64>,
    font: Option<FontData>,
    effects: EffectStore,
}

impl<A: Application + 'static> AppRunner<A> {
    fn new(user_app: A) -> Self {
        Self {
            user_app,
            window: None,
            window_id: None,
            render_cx: RenderContext::new(),
            surface: None,
            renderer: None,
            scene: Scene::new(),
            input: InputState::default(),
            active_button: None,
            font: load_default_font(),
            effects: EffectStore::new(),
        }
    }

    fn init_window_and_gpu(&mut self, event_loop: &ActiveEventLoop) {
        let attributes: WindowAttributes = Window::default_attributes()
            .with_title("runo example")
            .with_inner_size(LogicalSize::new(640.0, 480.0));

        let window = Arc::new(
            event_loop
                .create_window(attributes)
                .expect("failed to create window"),
        );
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

    fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        if let Some(surface) = self.surface.as_mut() {
            self.render_cx.resize_surface(surface, width, height);
        }
    }

    fn render(&mut self) {
        let Some(surface) = self.surface.as_mut() else {
            return;
        };
        let Some(renderer) = self.renderer.as_mut() else {
            return;
        };

        self.scene.reset();
        let bg = Rect::new(
            0.0,
            0.0,
            surface.config.width as f64,
            surface.config.height as f64,
        );
        self.scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            Color::from_rgb8(18, 20, 23),
            None,
            &bg,
        );

        self.effects.begin_frame();

        let frame_input = self.input.frame();
        {
            let mut ui = Ui::new(
                &mut self.scene,
                frame_input,
                &mut self.active_button,
                self.font.clone(),
                &mut self.effects,
            );
            self.user_app.update(&mut ui);
        }

        self.effects.end_frame();

        if frame_input.mouse_released {
            self.active_button = None;
        }

        self.input.end_frame();

        let dev_id = surface.dev_id;
        let device = &self.render_cx.devices[dev_id].device;
        let queue = &self.render_cx.devices[dev_id].queue;

        let surface_texture = match surface.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost) => {
                self.render_cx
                    .resize_surface(surface, surface.config.width, surface.config.height);
                return;
            }
            Err(wgpu::SurfaceError::Timeout) => return,
            Err(wgpu::SurfaceError::OutOfMemory) => panic!("out of memory"),
            Err(wgpu::SurfaceError::Other) => return,
        };

        renderer
            .render_to_texture(
                device,
                queue,
                &self.scene,
                &surface.target_view,
                &RenderParams {
                    base_color: Color::from_rgb8(0, 0, 0),
                    width: surface.config.width,
                    height: surface.config.height,
                    antialiasing_method: AaConfig::Msaa16,
                },
            )
            .expect("render failed");

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("runo surface blit"),
        });

        let surface_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        surface
            .blitter
            .copy(device, &mut encoder, &surface.target_view, &surface_view);

        queue.submit([encoder.finish()]);
        surface_texture.present();
    }

    fn request_redraw(&self) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

impl<A: Application + 'static> ApplicationHandler for AppRunner<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            self.init_window_and_gpu(event_loop);
            self.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if Some(window_id) != self.window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                self.resize(size.width, size.height);
                self.request_redraw();
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.input.set_cursor_pos(position.x, position.y);
                self.request_redraw();
            }
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => {
                self.input.on_mouse_input(state);
                self.request_redraw();
            }
            WindowEvent::RedrawRequested => self.render(),
            _ => {}
        }
    }
}
