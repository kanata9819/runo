use vello::util::{RenderContext, RenderSurface};
use vello::wgpu;
use vello::{AaConfig, RenderParams, Renderer, Scene};

use crate::Color;
use crate::app::{AppRunner, RunoApplication};

impl<A: RunoApplication + 'static> AppRunner<A> {
    pub(super) fn acquire_surface_texture(
        render_cx: &mut RenderContext,
        surface: &mut RenderSurface<'static>,
    ) -> Option<wgpu::SurfaceTexture> {
        match surface.surface.get_current_texture() {
            Ok(frame) => Some(frame),
            Err(wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost) => {
                render_cx.resize_surface(surface, surface.config.width, surface.config.height);
                None
            }
            Err(wgpu::SurfaceError::Timeout) => None,
            Err(wgpu::SurfaceError::OutOfMemory) => panic!("out of memory"),
            Err(wgpu::SurfaceError::Other) => None,
        }
    }

    pub(super) fn render_scene_to_target(
        renderer: &mut Renderer,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        scene: &Scene,
        target_view: &wgpu::TextureView,
        width: u32,
        height: u32,
    ) {
        renderer
            .render_to_texture(
                device,
                queue,
                scene,
                target_view,
                &RenderParams {
                    base_color: Color::from_rgb8(0, 0, 0),
                    width,
                    height,
                    antialiasing_method: AaConfig::Msaa16,
                },
            )
            .expect("render failed");
    }

    pub(super) fn blit_to_surface(
        surface: &RenderSurface<'static>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface_texture: wgpu::SurfaceTexture,
    ) {
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
}
