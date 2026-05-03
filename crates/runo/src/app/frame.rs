use vello::Scene;
use vello::kurbo::{Affine, Rect};
use vello::peniko::Fill;

use crate::Color;
use crate::app::{AppRunner, RunoApplication};
use crate::ui::Ui;

#[cfg(test)]
#[path = "../../tests/unit/app/frame.rs"]
mod tests;

impl<A, Event> AppRunner<A, Event>
where
    A: RunoApplication<Event> + 'static,
    Event: 'static,
{
    pub(super) fn render(&mut self) -> bool {
        let Some((physical_width, physical_height)) = self.surface_size() else {
            return false;
        };
        let Some((logical_width, logical_height)) = self.logical_surface_size() else {
            return false;
        };

        self.compose_frame(logical_width, logical_height);
        self.submit_frame(physical_width, physical_height)
    }

    fn surface_size(&self) -> Option<(u32, u32)> {
        let surface: &vello::util::RenderSurface<'static> = self.surface.as_ref()?;
        Some((surface.config.width, surface.config.height))
    }

    fn compose_frame(&mut self, width: f64, height: f64) {
        self.build_scene(width, height);
        self.run_ui_frame();
        self.retained.render(&mut self.scene, self.font.as_ref());
    }

    fn submit_frame(&mut self, width: u32, height: u32) -> bool {
        let scale_factor: f64 = self.scale_factor();
        let Some(surface) = self.surface.as_mut() else {
            return false;
        };

        let Some(renderer) = self.renderer.as_mut() else {
            return false;
        };

        let dev_id: usize = surface.dev_id;
        let surface_texture: vello::wgpu::SurfaceTexture =
            match Self::acquire_surface_texture(&mut self.render_cx, surface) {
                Ok(Some(texture)) => texture,
                Ok(None) => return false,
                Err(crate::app::gpu::GpuFatalError::OutOfMemory) => {
                    eprintln!("fatal gpu error: surface out of memory");
                    return true;
                }
            };

        let device: &vello::wgpu::Device = &self.render_cx.devices[dev_id].device;
        let queue: &vello::wgpu::Queue = &self.render_cx.devices[dev_id].queue;
        let mut scaled_scene: Scene = Scene::new();
        scaled_scene.append(&self.scene, Some(Affine::scale(scale_factor)));

        if let Err(err) = Self::render_scene_to_target(
            renderer,
            device,
            queue,
            &scaled_scene,
            &surface.target_view,
            width,
            height,
        ) {
            eprintln!("gpu render error: {err}");
            return false;
        }

        Self::blit_to_surface(surface, device, queue, surface_texture);
        false
    }

    fn build_scene(&mut self, width: f64, height: f64) {
        self.scene.reset();
        let bg: Rect = Rect::new(0.0, 0.0, width, height);
        self.scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            Color::from_rgb8(18, 20, 23),
            None,
            &bg,
        );
    }

    fn run_ui_frame(&mut self) {
        self.remount_if_needed();
        self.retained
            .begin_frame_input(self.input.snapshot(), self.font.as_ref());
        let request_remount: bool = self.run_app_update() || self.dispatch_bound_events();
        self.apply_frame_updates(request_remount);
        self.input.end_frame();
    }

    fn remount_if_needed(&mut self) {
        if !self.mount_required {
            return;
        }

        self.effects.begin_frame();
        self.states.begin_frame();
        self.retained.begin_build_pass();

        let bindings: crate::ui::EventBindings<Event> = {
            let input = crate::input::UiInputSnapshot::from(&self.input.snapshot());
            let mut ui = Ui::with_input(
                &mut self.scene,
                self.font.clone(),
                input,
                &mut self.effects,
                &mut self.states,
                &mut self.retained,
            );

            self.user_app.build(&mut ui)
        };

        self.bindings = bindings;
        self.retained.prune_unseen_widgets();
        self.effects.end_frame();
        self.states.end_frame();
        self.mount_required = false;
    }

    fn run_app_update(&mut self) -> bool {
        let input = crate::input::UiInputSnapshot::from(&self.input.snapshot());
        let mut ui = Ui::with_input(
            &mut self.scene,
            self.font.clone(),
            input,
            &mut self.effects,
            &mut self.states,
            &mut self.retained,
        );

        self.user_app.update(&mut ui)
    }

    fn dispatch_bound_events(&mut self) -> bool {
        let mut request_remount: bool = false;
        {
            let input = crate::input::UiInputSnapshot::from(&self.input.snapshot());
            let mut ui = Ui::with_input(
                &mut self.scene,
                self.font.clone(),
                input,
                &mut self.effects,
                &mut self.states,
                &mut self.retained,
            );

            for event in ui.drain_bound_events(&self.bindings) {
                request_remount |= self.user_app.on_event(&mut ui, event);
            }
        }

        request_remount
    }

    fn apply_frame_updates(&mut self, request_remount: bool) {
        let state_changed: bool = self.states.take_changed();
        if request_remount || state_changed {
            self.mount_required = true;
        }

        if request_remount || state_changed || self.window_options.immediate_mode {
            self.request_redraw();
        }
    }
}
