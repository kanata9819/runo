use runo_core::{Application, RunOptions, Ui, run};

struct MyApp {
    toggled: bool,
}

impl Application for MyApp {
    fn options(&self) -> RunOptions {
        RunOptions {
            window_title: "runo example".to_string(),
            window_width: 1200,
            window_height: 700,
        }
    }

    fn build(&mut self, ui: &mut Ui<'_>) {
        ui.vertical(|ui| {
            ui.label("runo example").size(22).show();
            ui.button_id("btnToggle")
                .width(220)
                .height(64)
                .text("Toggle: OFF")
                .show();
        });
    }

    fn update(&mut self, ui: &mut Ui<'_>) {
        if ui.button_clicked("btnToggle") {
            self.toggled = !self.toggled;
            let next_text = if self.toggled {
                "Toggle: ON"
            } else {
                "Toggle: OFF"
            };
            ui.set_button_text("btnToggle", next_text);
        }

        let toggled = self.toggled;
        ui.use_effect("toggle_effect", toggled, move || {
            println!("toggled changed: {}", toggled);
            None
        });
    }
}

fn main() {
    run(MyApp { toggled: false });
}
