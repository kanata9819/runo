use vello::kurbo::{Affine, Rect};
use vello::peniko::Fill;

use crate::Color;
use crate::app::{AppRunner, Application};
use crate::ui::Ui;

impl<A: Application + 'static> AppRunner<A> {
    pub(super) fn render(&mut self) {
        let Some((width, height)) = self.surface_size() else {
            return;
        };

        self.compose_frame(width, height);
        self.submit_frame(width, height);
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

    fn submit_frame(&mut self, width: u32, height: u32) {
        let Some(surface) = self.surface.as_mut() else {
            return;
        };
        let Some(renderer) = self.renderer.as_mut() else {
            return;
        };

        let dev_id = surface.dev_id;
        let Some(surface_texture) = Self::acquire_surface_texture(&mut self.render_cx, surface)
        else {
            return;
        };
        let device = &self.render_cx.devices[dev_id].device;
        let queue = &self.render_cx.devices[dev_id].queue;

        Self::render_scene_to_target(
            renderer,
            device,
            queue,
            &self.scene,
            &surface.target_view,
            width,
            height,
        );
        Self::blit_to_surface(surface, device, queue, surface_texture);
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

        if !self.built {
            let mut ui = Ui::new(
                &mut self.scene,
                self.font.clone(),
                &mut self.effects,
                &mut self.retained,
            );
            self.user_app.build(&mut ui);
            self.built = true;
        }

        self.retained.begin_frame_input(self.input.frame());
        {
            let mut ui = Ui::new(
                &mut self.scene,
                self.font.clone(),
                &mut self.effects,
                &mut self.retained,
            );
            self.user_app.update(&mut ui);
        }

        self.effects.end_frame();
        self.input.end_frame();
    }
}
