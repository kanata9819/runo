use vello::wgpu;
use vello::{AaConfig, RenderParams};

use crate::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SurfaceAcquireAction {
    SkipFrame,
    FatalOutOfMemory,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum GpuFatalError {
    OutOfMemory,
}

pub(super) fn render_params(width: u32, height: u32) -> RenderParams {
    RenderParams {
        base_color: Color::from_rgb8(0, 0, 0),
        width,
        height,
        antialiasing_method: AaConfig::Msaa16,
    }
}

pub(super) fn map_surface_error(error: &wgpu::SurfaceError) -> SurfaceAcquireAction {
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
