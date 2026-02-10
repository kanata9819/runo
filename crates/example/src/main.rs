use runo_core::{Application, RunOptions, Ui, UiEvent, run};

struct MyApp {
    toggled: bool,
    input_text: String,
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
            ui.text_box_id("input.name")
                .size(320, 44)
                .placeholder("Type here...")
                .show();
            ui.button_id("btnToggle")
                .width(220)
                .height(64)
                .text("Toggle: OFF")
                .show();
        });
    }

    fn update(&mut self, ui: &mut Ui<'_>) {
        for event in ui.drain_events() {
            match event {
                UiEvent::ButtonClicked { id } if id == "btnToggle" => {
                    self.toggled = !self.toggled;
                    let label = if self.toggled {
                        "Toggle: ON"
                    } else {
                        "Toggle: OFF"
                    };
                    if self.input_text.is_empty() {
                        ui.set_button_text("btnToggle", label);
                    } else {
                        ui.set_button_text("btnToggle", format!("{} ({})", label, self.input_text));
                    }
                }
                UiEvent::TextBoxChanged { id, text } if id == "input.name" => {
                    self.input_text = text;
                    let label = if self.toggled {
                        "Toggle: ON"
                    } else {
                        "Toggle: OFF"
                    };
                    if self.input_text.is_empty() {
                        ui.set_button_text("btnToggle", label);
                    } else {
                        ui.set_button_text("btnToggle", format!("{} ({})", label, self.input_text));
                    }
                }
                _ => {}
            }
        }

        let toggled = self.toggled;
        ui.use_effect("toggle_effect", toggled, move || {
            println!("toggled changed: {}", toggled);
            None
        });
    }
}

fn main() {
    run(MyApp {
        toggled: false,
        input_text: String::new(),
    });
}
