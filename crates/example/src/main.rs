use runo_core::{Application, Color, RunOptions, Ui, UiEvent, run};

const TITLE_ID: &str = "title";
const INPUT_NAME_ID: &str = "input.name";
const COMBO_ROLE_ID: &str = "combo.role";
const TOGGLE_BUTTON_ID: &str = "btnToggle";

struct MyApp {
    toggled: bool,
    input_text: String,
    selected_role: String,
}

impl MyApp {
    fn build_title(ui: &mut Ui<'_>) {
        ui.widgets()
            .label()
            .id(TITLE_ID)
            .text("runo example")
            .font_size(22)
            .show();
    }

    fn build_name_input(ui: &mut Ui<'_>) {
        ui.widgets()
            .text_box()
            .id(INPUT_NAME_ID)
            .width(320)
            .height(44)
            .font_size(20)
            .placeholder("Type here...")
            .overflow_x(runo_core::Overflow::Scroll)
            .show();
    }

    fn build_role_combo(ui: &mut Ui<'_>) {
        ui.widgets()
            .combo_box()
            .id(COMBO_ROLE_ID)
            .width(320)
            .height(44)
            .font_size(18)
            .items(["Designer", "Engineer", "Manager"])
            .show();
    }

    fn build_toggle_button(ui: &mut Ui<'_>) {
        ui.widgets()
            .button()
            .id(TOGGLE_BUTTON_ID)
            .width(220)
            .height(64)
            .font_size(18)
            .text("Toggle: OFF")
            .show();
    }
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
            Self::build_title(ui);
            ui.widgets()
                .div()
                .id("main.panel")
                .width(380)
                .padding(16)
                .gap(10)
                .background(Color::from_rgb8(29, 34, 41))
                .border(Color::from_rgb8(70, 80, 95), 1)
                .radius(12)
                .show(|ui| {
                    Self::build_name_input(ui);
                    Self::build_role_combo(ui);
                    Self::build_toggle_button(ui);
                });
        });
    }

    fn update(&mut self, ui: &mut Ui<'_>) {
        for event in ui.events().drain_events() {
            match event {
                UiEvent::ButtonClicked { id } if id == TOGGLE_BUTTON_ID => {
                    self.toggled = !self.toggled;
                    let label = if self.toggled {
                        "Toggle: ON"
                    } else {
                        "Toggle: OFF"
                    };
                    if self.input_text.is_empty() {
                        ui.state().button().set_text(TOGGLE_BUTTON_ID, label);
                    } else {
                        ui.state()
                            .button()
                            .set_text(TOGGLE_BUTTON_ID, format!("{} ({})", label, self.input_text));
                    }
                }
                UiEvent::TextBoxChanged { id, text } if id == INPUT_NAME_ID => {
                    self.input_text = text;
                    let label = if self.toggled {
                        "Toggle: ON"
                    } else {
                        "Toggle: OFF"
                    };
                    if self.input_text.is_empty() {
                        ui.state().button().set_text(TOGGLE_BUTTON_ID, label);
                    } else {
                        ui.state()
                            .button()
                            .set_text(TOGGLE_BUTTON_ID, format!("{} ({})", label, self.input_text));
                    }
                }
                UiEvent::ComboBoxChanged {
                    id, selected_text, ..
                } if id == COMBO_ROLE_ID => {
                    self.selected_role = selected_text;
                    let label = if self.toggled {
                        "Toggle: ON"
                    } else {
                        "Toggle: OFF"
                    };
                    if self.input_text.is_empty() {
                        ui.state().button().set_text(
                            TOGGLE_BUTTON_ID,
                            format!("{} [{}]", label, self.selected_role),
                        );
                        println!("{}", ui.state().combo_box().selected_index(COMBO_ROLE_ID));
                    } else {
                        ui.state().button().set_text(
                            TOGGLE_BUTTON_ID,
                            format!("{} ({}) [{}]", label, self.input_text, self.selected_role),
                        );
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
        selected_role: "Designer".to_string(),
    });
}
