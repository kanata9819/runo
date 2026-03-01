
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
