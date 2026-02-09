use std::fs;
use std::num::NonZero;
use std::sync::Arc;

use vello::kurbo::{Affine, Rect};
pub use vello::peniko::Color;
use vello::peniko::{Blob, Fill, FontData};
use vello::util::{RenderContext, RenderSurface};
use vello::wgpu;
use vello::{AaConfig, AaSupport, RenderParams, Renderer, RendererOptions, Scene};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

mod widget;
use crate::widget::button::ButtonBuilder;
pub use crate::widget::button::ButtonResponse;

pub trait Application {
    fn update(&mut self, ui: &mut Ui<'_>);
}

#[derive(Clone, Copy)]
enum LayoutDirection {
    Vertical,
    Horizontal,
}

struct LayoutNode {
    origin: (f64, f64),
    cursor: f64,
    cross: f64,
    direction: LayoutDirection,
    spacing: f64,
}

impl LayoutNode {
    fn new(origin: (f64, f64), direction: LayoutDirection, spacing: f64) -> Self {
        Self {
            origin,
            cursor: 0.0,
            cross: 0.0,
            direction,
            spacing,
        }
    }

    fn place(&self, _width: f64, _height: f64) -> (f64, f64) {
        match self.direction {
            LayoutDirection::Vertical => (self.origin.0, self.origin.1 + self.cursor),
            LayoutDirection::Horizontal => (self.origin.0 + self.cursor, self.origin.1),
        }
    }

    fn advance(&mut self, width: f64, height: f64) {
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

    fn consumed_size(&self) -> (f64, f64) {
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

#[derive(Clone, Copy)]
pub(crate) struct InputFrame {
    pub(crate) cursor_pos: (f64, f64),
    pub(crate) mouse_down: bool,
    pub(crate) mouse_pressed: bool,
    pub(crate) mouse_released: bool,
}

#[derive(Default)]
struct InputState {
    cursor_pos: (f64, f64),
    mouse_down: bool,
    mouse_pressed: bool,
    mouse_released: bool,
}

impl InputState {
    fn frame(&self) -> InputFrame {
        InputFrame {
            cursor_pos: self.cursor_pos,
            mouse_down: self.mouse_down,
            mouse_pressed: self.mouse_pressed,
            mouse_released: self.mouse_released,
        }
    }

    fn end_frame(&mut self) {
        self.mouse_pressed = false;
        self.mouse_released = false;
    }

    fn on_mouse_input(&mut self, state: ElementState) {
        let next = state == ElementState::Pressed;
        if next && !self.mouse_down {
            self.mouse_pressed = true;
        }
        if !next && self.mouse_down {
            self.mouse_released = true;
        }
        self.mouse_down = next;
    }
}

pub struct Ui<'a> {
    pub(crate) scene: &'a mut Scene,
    pub(crate) input: InputFrame,
    pub(crate) active_button: &'a mut Option<u64>,
    pub(crate) font: Option<FontData>,
    layout_stack: Vec<LayoutNode>,
    auto_id_counter: u64,
}

impl<'a> Ui<'a> {
    pub fn button<'ui>(&'ui mut self) -> ButtonBuilder<'ui, 'a> {
        let id = format!("__auto_button_{}", self.auto_id_counter);
        self.auto_id_counter += 1;
        ButtonBuilder::new(self, id)
    }

    pub fn button_id<'ui>(&'ui mut self, id: impl Into<String>) -> ButtonBuilder<'ui, 'a> {
        ButtonBuilder::new(self, id.into())
    }

    pub fn vertical<R>(&mut self, f: impl FnOnce(&mut Ui<'a>) -> R) -> R {
        self.with_layout(LayoutDirection::Vertical, 12.0, f)
    }

    pub fn horizontal<R>(&mut self, f: impl FnOnce(&mut Ui<'a>) -> R) -> R {
        self.with_layout(LayoutDirection::Horizontal, 12.0, f)
    }

    pub fn fill_rect(&mut self, x: f64, y: f64, w: f64, h: f64, color: Color) {
        let rect = Rect::new(x, y, x + w, y + h);
        self.scene
            .fill(Fill::NonZero, Affine::IDENTITY, color, None, &rect);
    }

    fn with_layout<R>(
        &mut self,
        direction: LayoutDirection,
        spacing: f64,
        f: impl FnOnce(&mut Ui<'a>) -> R,
    ) -> R {
        let origin = {
            let parent = self.layout_stack.last().expect("layout stack is empty");
            parent.place(0.0, 0.0)
        };
        self.layout_stack
            .push(LayoutNode::new(origin, direction, spacing));
        let result = f(self);
        let child = self.layout_stack.pop().expect("layout stack underflow");
        let (cw, ch) = child.consumed_size();
        self.advance_layout(cw, ch);
        result
    }

    pub(crate) fn allocate_rect(&mut self, width: f64, height: f64) -> (f64, f64) {
        let pos = {
            let layout = self.layout_stack.last().expect("layout stack is empty");
            layout.place(width, height)
        };
        self.advance_layout(width, height);
        pos
    }

    fn advance_layout(&mut self, width: f64, height: f64) {
        if let Some(layout) = self.layout_stack.last_mut() {
            layout.advance(width, height);
        }
    }
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

        let frame_input = self.input.frame();
        {
            let mut ui = Ui {
                scene: &mut self.scene,
                input: frame_input,
                active_button: &mut self.active_button,
                font: self.font.clone(),
                layout_stack: vec![LayoutNode::new(
                    (24.0, 24.0),
                    LayoutDirection::Vertical,
                    12.0,
                )],
                auto_id_counter: 0,
            };
            self.user_app.update(&mut ui);
        }

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
                self.input.cursor_pos = (position.x, position.y);
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

fn load_default_font() -> Option<FontData> {
    const CANDIDATES: &[&str] = &[
        "C:\\Windows\\Fonts\\segoeui.ttf",
        "C:\\Windows\\Fonts\\arial.ttf",
        "/mnt/c/Windows/Fonts/segoeui.ttf",
        "/mnt/c/Windows/Fonts/arial.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    ];

    for path in CANDIDATES {
        if let Some(font) = load_font_from_path(path) {
            return Some(font);
        }
    }
    None
}

fn load_font_from_path(path: &str) -> Option<FontData> {
    let bytes = fs::read(path).ok()?;
    let blob = Blob::new(Arc::new(bytes.into_boxed_slice()));
    Some(FontData::new(blob, 0))
}
