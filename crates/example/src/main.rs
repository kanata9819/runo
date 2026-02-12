#![windows_subsystem = "windows"]

use runo_core::{RunOptions, RunoApplication, Ui, UiEvent, colors, run};

const TITLE_ID: &str = "title";
const INPUT_NAME_ID: &str = "input.name";
const COMBO_ROLE_ID: &str = "combo.role";
const CHECKBOX_NEWSLETTER_ID: &str = "check.newsletter";
const RADIO_CHANNEL_EMAIL_ID: &str = "radio.channel.email";
const RADIO_CHANNEL_SMS_ID: &str = "radio.channel.sms";
const RADIO_CHANNEL_PUSH_ID: &str = "radio.channel.push";
const RADIO_CHANNEL_GROUP: &str = "channel";
const SLIDER_VOLUME_ID: &str = "slider.volume";
const TOGGLE_BUTTON_ID: &str = "btnToggle";
const MAIN_PANEL_ID: &str = "main.panel";

struct MyApp {
    toggled: bool,
    input_text: String,
    selected_role: String,
    newsletter_opt_in: bool,
    selected_channel: String,
    volume: f64,
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
            .overflow_x(runo_core::Overflow::Auto)
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

    fn build_newsletter_checkbox(ui: &mut Ui<'_>) {
        ui.widgets()
            .checkbox()
            .id(CHECKBOX_NEWSLETTER_ID)
            .height(40)
            .text("Receive newsletter")
            .checked(true)
            .show();
    }

    fn build_channel_radio_buttons(ui: &mut Ui<'_>) {
        ui.widgets()
            .radio_button()
            .id(RADIO_CHANNEL_EMAIL_ID)
            .group(RADIO_CHANNEL_GROUP)
            .height(36)
            .text("Channel: Email")
            .selected(true)
            .show();
        ui.widgets()
            .radio_button()
            .id(RADIO_CHANNEL_SMS_ID)
            .group(RADIO_CHANNEL_GROUP)
            .height(36)
            .text("Channel: SMS")
            .show();
        ui.widgets()
            .radio_button()
            .id(RADIO_CHANNEL_PUSH_ID)
            .group(RADIO_CHANNEL_GROUP)
            .height(36)
            .text("Channel: Push")
            .show();
    }

    fn build_volume_slider(ui: &mut Ui<'_>) {
        ui.widgets()
            .slider()
            .id(SLIDER_VOLUME_ID)
            .width(320)
            .height(48)
            .text("Volume")
            .range(0.0, 1.0)
            .step(0.01)
            .value(0.35)
            .show();
    }
}

impl RunoApplication for MyApp {
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
                .id(MAIN_PANEL_ID)
                .width(380)
                .padding(16)
                .gap(10)
                .background(colors::rgb(colors::PANEL_BG))
                .border(colors::rgb(colors::PANEL_BORDER), 1)
                .radius(12)
                .show(|ui| {
                    Self::build_name_input(ui);
                    Self::build_role_combo(ui);
                    Self::build_newsletter_checkbox(ui);
                    Self::build_channel_radio_buttons(ui);
                    Self::build_volume_slider(ui);
                    Self::build_toggle_button(ui);
                });
        });
    }

    fn update(&mut self, ui: &mut Ui<'_>) {
        for event in ui.events().drain_events() {
            match event {
                UiEvent::ButtonClicked { id } if id == TOGGLE_BUTTON_ID => {
                    self.toggled = !self.toggled;
                    let panel_color = if self.toggled {
                        colors::rgb(colors::PANEL_BG_ACTIVE)
                    } else {
                        colors::rgb(colors::PANEL_BG)
                    };
                    ui.state().div().set_background(MAIN_PANEL_ID, panel_color);
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
                UiEvent::CheckboxChanged { id, checked } if id == CHECKBOX_NEWSLETTER_ID => {
                    self.newsletter_opt_in = checked;
                    let label = if self.toggled {
                        "Toggle: ON"
                    } else {
                        "Toggle: OFF"
                    };
                    ui.state().button().set_text(
                        TOGGLE_BUTTON_ID,
                        format!(
                            "{} ({}) [{}] {}",
                            label,
                            if self.input_text.is_empty() {
                                "anonymous"
                            } else {
                                &self.input_text
                            },
                            self.selected_role,
                            if self.newsletter_opt_in {
                                "newsletter:on"
                            } else {
                                "newsletter:off"
                            }
                        ),
                    );
                }
                UiEvent::RadioButtonChanged { id, group, .. } if group == RADIO_CHANNEL_GROUP => {
                    self.selected_channel = match id.as_str() {
                        RADIO_CHANNEL_EMAIL_ID => "Email".to_string(),
                        RADIO_CHANNEL_SMS_ID => "SMS".to_string(),
                        RADIO_CHANNEL_PUSH_ID => "Push".to_string(),
                        _ => self.selected_channel.clone(),
                    };
                    println!("selected channel: {}", self.selected_channel);
                }
                UiEvent::SliderChanged { id, value } if id == SLIDER_VOLUME_ID => {
                    self.volume = value;
                    println!("volume: {:.2}", self.volume);
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
        newsletter_opt_in: true,
        selected_channel: "Email".to_string(),
        volume: 0.35,
    });
}
