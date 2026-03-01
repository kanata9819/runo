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
#[path = "../../tests/unit/app/gpu.rs"]
mod tests;
