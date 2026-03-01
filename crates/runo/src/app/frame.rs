use vello::kurbo::{Affine, Rect};
use vello::peniko::Fill;

use crate::Color;
use crate::app::{AppRunner, RunoApplication};
use crate::ui::Ui;

#[cfg(test)]
#[path = "../../tests/unit/app/frame.rs"]
mod tests;

impl<A: RunoApplication + 'static> AppRunner<A> {
    pub(super) fn render(&mut self) -> bool {
        let Some((width, height)) = self.surface_size() else {
            return false;
        };

        self.compose_frame(width, height);
        self.submit_frame(width, height)
    }

    fn surface_size(&self) -> Option<(u32, u32)> {
        let surface = self.surface.as_ref()?;
        Some((surface.config.width, surface.config.height))
    }

    fn compose_frame(&mut self, width: u32, height: u32) {
        self.build_scene(width, height);
        self.run_ui_frame();
        self.retained.render(&mut self.scene, self.font.as_ref());
    }

    fn submit_frame(&mut self, width: u32, height: u32) -> bool {
        let Some(surface) = self.surface.as_mut() else {
            return false;
        };

        let Some(renderer) = self.renderer.as_mut() else {
            return false;
        };

        let dev_id = surface.dev_id;
        let surface_texture = match Self::acquire_surface_texture(&mut self.render_cx, surface) {
            Ok(Some(texture)) => texture,
            Ok(None) => return false,
            Err(crate::app::gpu::GpuFatalError::OutOfMemory) => {
                eprintln!("fatal gpu error: surface out of memory");
                return true;
            }
        };

        let device = &self.render_cx.devices[dev_id].device;
        let queue = &self.render_cx.devices[dev_id].queue;

        if let Err(err) = Self::render_scene_to_target(
            renderer,
            device,
            queue,
            &self.scene,
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

    fn build_scene(&mut self, width: u32, height: u32) {
        self.scene.reset();
        let bg = Rect::new(0.0, 0.0, width as f64, height as f64);
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
        let request_remount = self.dispatch_bound_events();
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

        let bindings = {
            let mut ui = Ui::new(
                &mut self.scene,
                self.font.clone(),
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

    fn dispatch_bound_events(&mut self) -> bool {
        let mut request_remount = false;
        {
            let mut ui = Ui::new(
                &mut self.scene,
                self.font.clone(),
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
        if request_remount || self.states.take_changed() {
            self.mount_required = true;
            self.request_redraw();
        }
    }
}
