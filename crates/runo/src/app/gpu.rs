use vello::util::{RenderContext, RenderSurface};
use vello::wgpu;
use vello::{AaConfig, RenderParams, Renderer, Scene};

use crate::Color;
use crate::app::{AppRunner, RunoApplication};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SurfaceAcquireAction {
    SkipFrame,
    FatalOutOfMemory,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum GpuFatalError {
    OutOfMemory,
}

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

fn render_params(width: u32, height: u32) -> RenderParams {
    RenderParams {
        base_color: Color::from_rgb8(0, 0, 0),
        width,
        height,
        antialiasing_method: AaConfig::Msaa16,
    }
}

fn map_surface_error(error: &wgpu::SurfaceError) -> SurfaceAcquireAction {
    match error {
        wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost => SurfaceAcquireAction::SkipFrame,
        wgpu::SurfaceError::Timeout | wgpu::SurfaceError::Other => SurfaceAcquireAction::SkipFrame,
        wgpu::SurfaceError::OutOfMemory => SurfaceAcquireAction::FatalOutOfMemory,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_error_mapping_marks_out_of_memory_as_fatal() {
        assert_eq!(
            map_surface_error(&wgpu::SurfaceError::OutOfMemory),
            SurfaceAcquireAction::FatalOutOfMemory
        );
    }

    #[test]
    fn surface_error_mapping_skips_frame_for_recoverable_errors() {
        assert_eq!(
            map_surface_error(&wgpu::SurfaceError::Lost),
            SurfaceAcquireAction::SkipFrame
        );
        assert_eq!(
            map_surface_error(&wgpu::SurfaceError::Outdated),
            SurfaceAcquireAction::SkipFrame
        );
        assert_eq!(
            map_surface_error(&wgpu::SurfaceError::Timeout),
            SurfaceAcquireAction::SkipFrame
        );
        assert_eq!(
            map_surface_error(&wgpu::SurfaceError::Other),
            SurfaceAcquireAction::SkipFrame
        );
    }

    #[test]
    fn render_params_use_expected_defaults() {
        let params = render_params(640, 480);
        assert_eq!(params.width, 640);
        assert_eq!(params.height, 480);
        assert_eq!(params.base_color, Color::from_rgb8(0, 0, 0));
        assert!(matches!(params.antialiasing_method, AaConfig::Msaa16));
    }
}
