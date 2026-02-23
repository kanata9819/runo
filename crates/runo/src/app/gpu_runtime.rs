use vello::util::{RenderContext, RenderSurface};
use vello::wgpu;
use vello::{Renderer, Scene};

use crate::app::gpu::{GpuFatalError, SurfaceAcquireAction, map_surface_error, render_params};
use crate::app::{AppRunner, RunoApplication};

impl<A: RunoApplication + 'static> AppRunner<A> {
    pub(super) fn acquire_surface_texture(
        render_cx: &mut RenderContext,
        surface: &mut RenderSurface<'static>,
    ) -> Result<Option<wgpu::SurfaceTexture>, GpuFatalError> {
        match surface.surface.get_current_texture() {
            Ok(frame) => Ok(Some(frame)),
            Err(err) => match map_surface_error(&err) {
                SurfaceAcquireAction::SkipFrame => {
                    if matches!(err, wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost) {
                        render_cx.resize_surface(
                            surface,
                            surface.config.width,
                            surface.config.height,
                        );
                    }
                    Ok(None)
                }
                SurfaceAcquireAction::FatalOutOfMemory => Err(GpuFatalError::OutOfMemory),
            },
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
    ) -> Result<(), String> {
        renderer
            .render_to_texture(
                device,
                queue,
                scene,
                target_view,
                &render_params(width, height),
            )
            .map_err(|err| format!("{err:?}"))
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
