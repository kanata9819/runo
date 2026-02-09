use runo_core::{Application, Color, Ui, run};

struct MyApp {
    toggled: bool,
}

impl Application for MyApp {
    fn update(&mut self, ui: &mut Ui<'_>) {
        let toggled = self.toggled;
        ui.use_effect("toggle_effect", toggled, move || {
            println!("toggled changed: {}", toggled);
            None
        });

        ui.vertical(|ui| {
            let button = ui
                .button_id("main.toggle")
                .width(220.0)
                .height(64.0)
                .text("Toggle")
                .show();

            if button.clicked {
                self.toggled = !self.toggled;
            }

            if self.toggled {
                ui.fill_rect(24.0, 120.0, 220.0, 40.0, Color::from_rgb8(234, 197, 92));
            }
        });
    }
}

fn main() {
    run(MyApp { toggled: false });
}
