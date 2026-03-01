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
        self.effects.begin_frame();
        self.states.begin_frame();

        {
            let mut ui = Ui::new(
                &mut self.scene,
                self.font.clone(),
                &mut self.effects,
                &mut self.states,
                &mut self.retained,
            );

            self.user_app.build(&mut ui);
        }

        self.retained
            .begin_frame_input(self.input.snapshot(), self.font.as_ref());

        {
            let mut ui = Ui::new(
                &mut self.scene,
                self.font.clone(),
                &mut self.effects,
                &mut self.states,
                &mut self.retained,
            );

            let bindings = self.user_app.event_bindings();
            for event in ui.drain_bound_events(&bindings) {
                self.user_app.on_event(&mut ui, event);
            }
        }

        self.effects.end_frame();
        self.states.end_frame();

        if self.states.take_changed() {
            self.request_redraw();
        }

        self.input.end_frame();
    }
}
