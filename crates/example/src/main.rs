use runo_core::{Application, Ui, run};

struct MyApp {
    toggled: bool,
}

impl Application for MyApp {
    fn build(&mut self, ui: &mut Ui<'_>) {
        ui.vertical(|ui| {
            ui.label("runo example").size(22.0).show();
            ui.button_id("main.toggle")
                .width(220.0)
                .height(64.0)
                .text("Toggle: OFF")
                .show();
        });
    }

    fn update(&mut self, ui: &mut Ui<'_>) {
        if ui.button_clicked("main.toggle") {
            self.toggled = !self.toggled;
            let next_text = if self.toggled {
                "Toggle: ON"
            } else {
                "Toggle: OFF"
            };
            ui.set_button_text("main.toggle", next_text);
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
