#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use runo::{
    ButtonHandle, CheckboxHandle, ComboBoxHandle, DivHandle, RadioButtonHandle, RunOptions,
    RunoApplication, SliderHandle, TextBoxHandle, Ui, colors, run,
};

#[derive(Clone)]
enum Event {
    NameChanged(String),
    RoleChanged { index: usize, text: String },
    NewsletterChanged(bool),
    ChannelEmailChanged(bool),
    ChannelSmsChanged(bool),
    ChannelPushChanged(bool),
    VolumeChanged(f64),
    ToggleClicked,
}

struct MyApp {
    toggled: bool,
    input_text: String,
    selected_role: String,
    newsletter_opt_in: bool,
    selected_channel: String,
    volume: f64,
    input_name: Option<TextBoxHandle>,
    role_combo: Option<ComboBoxHandle>,
    newsletter_checkbox: Option<CheckboxHandle>,
    channel_email: Option<RadioButtonHandle>,
    channel_sms: Option<RadioButtonHandle>,
    channel_push: Option<RadioButtonHandle>,
    volume_slider: Option<SliderHandle>,
    toggle_button: Option<ButtonHandle>,
    main_panel: Option<DivHandle>,
}

impl MyApp {
    fn build_title(ui: &mut Ui<'_>) {
        ui.widgets()
            .label()
            .text("runo example")
            .font_size(22)
            .show();
    }

    fn build_name_input(ui: &mut Ui<'_>) -> TextBoxHandle {
        ui.widgets()
            .text_box()
            .width(320)
            .height(44)
            .font_size(20)
            .placeholder("Type here...")
            .overflow_x(runo::Overflow::Auto)
            .show()
    }

    fn build_role_combo(ui: &mut Ui<'_>) -> ComboBoxHandle {
        ui.widgets()
            .combo_box()
            .width(320)
            .height(44)
            .font_size(18)
            .items(["Designer", "Engineer", "Manager"])
            .show()
    }

    fn build_toggle_button(ui: &mut Ui<'_>) -> ButtonHandle {
        ui.widgets()
            .button()
            .width(220)
            .height(64)
            .font_size(18)
            .text("Toggle: OFF")
            .show()
    }

    fn build_newsletter_checkbox(ui: &mut Ui<'_>) -> CheckboxHandle {
        ui.widgets()
            .checkbox()
            .height(40)
            .text("Receive newsletter")
            .checked(true)
            .show()
    }

    fn build_channel_radio_buttons(
        ui: &mut Ui<'_>,
    ) -> (RadioButtonHandle, RadioButtonHandle, RadioButtonHandle) {
        let email = ui
            .widgets()
            .radio_button()
            .group("channel")
            .height(36)
            .text("Channel: Email")
            .selected(true)
            .show();
        let sms = ui
            .widgets()
            .radio_button()
            .group("channel")
            .height(36)
            .text("Channel: SMS")
            .show();
        let push = ui
            .widgets()
            .radio_button()
            .group("channel")
            .height(36)
            .text("Channel: Push")
            .show();
        (email, sms, push)
    }

    fn build_volume_slider(ui: &mut Ui<'_>) -> SliderHandle {
        ui.widgets()
            .slider()
            .width(320)
            .height(48)
            .text("Volume")
            .range(0.0, 1.0)
            .step(0.01)
            .value(0.35)
            .show()
    }

    fn refresh_toggle_button(&self, ui: &mut Ui<'_>) {
        if let Some(toggle_button) = &self.toggle_button {
            let label = if self.toggled {
                "Toggle: ON"
            } else {
                "Toggle: OFF"
            };
            if self.newsletter_opt_in {
                toggle_button.set_text(
                    ui,
                    format!(
                        "{} ({}) [{}]",
                        label,
                        if self.input_text.is_empty() {
                            "anonymous"
                        } else {
                            &self.input_text
                        },
                        self.selected_role,
                    ),
                );
            } else {
                toggle_button.set_text(ui, label);
            }
        }
    }

    fn update_panel_color(&self, ui: &mut Ui<'_>) {
        if let Some(main_panel) = &self.main_panel {
            let panel_color = if self.toggled {
                colors::rgb(colors::PANEL_BG_ACTIVE)
            } else {
                colors::rgb(colors::PANEL_BG)
            };
            main_panel.set_background(ui, panel_color);
        }
    }
}

impl RunoApplication for MyApp {
    type Event = Event;

    fn options(&self) -> RunOptions {
        RunOptions {
            window_title: "runo example".to_string(),
            window_width: 1200,
            window_height: 700,
            window_resizable: true,
        }
    }

    fn build(&mut self, ui: &mut Ui<'_>) {
        ui.vertical(|ui| {
            Self::build_title(ui);
            let (main_panel, _) = ui
                .widgets()
                .div()
                .width(380)
                .padding(16)
                .gap(10)
                .background(colors::rgb(colors::PANEL_BG))
                .border(colors::rgb(colors::PANEL_BORDER), 1)
                .radius(12)
                .show_with_handle(|ui| {
                    self.input_name = Some(Self::build_name_input(ui));
                    self.role_combo = Some(Self::build_role_combo(ui));
                    self.newsletter_checkbox = Some(Self::build_newsletter_checkbox(ui));
                    let (email, sms, push) = Self::build_channel_radio_buttons(ui);
                    self.channel_email = Some(email);
                    self.channel_sms = Some(sms);
                    self.channel_push = Some(push);
                    self.volume_slider = Some(Self::build_volume_slider(ui));
                    self.toggle_button = Some(Self::build_toggle_button(ui));
                });
            self.main_panel = Some(main_panel);
        });
    }

    fn event_bindings(&self) -> runo::EventBindings<Self::Event> {
        let mut builder = runo::EventBindings::builder();
        if let Some(input_name) = &self.input_name {
            builder = builder.text_box(input_name.clone(), Event::NameChanged);
        }
        if let Some(role_combo) = &self.role_combo {
            builder = builder.combo_box(role_combo.clone(), |index, text| Event::RoleChanged {
                index,
                text,
            });
        }
        if let Some(newsletter_checkbox) = &self.newsletter_checkbox {
            builder = builder.checkbox(newsletter_checkbox.clone(), Event::NewsletterChanged);
        }
        if let Some(channel_email) = &self.channel_email {
            builder = builder.radio_button(channel_email.clone(), Event::ChannelEmailChanged);
        }
        if let Some(channel_sms) = &self.channel_sms {
            builder = builder.radio_button(channel_sms.clone(), Event::ChannelSmsChanged);
        }
        if let Some(channel_push) = &self.channel_push {
            builder = builder.radio_button(channel_push.clone(), Event::ChannelPushChanged);
        }
        if let Some(volume_slider) = &self.volume_slider {
            builder = builder.slider(volume_slider.clone(), Event::VolumeChanged);
        }
        if let Some(toggle_button) = &self.toggle_button {
            builder = builder.button(toggle_button.clone(), Event::ToggleClicked);
        }
        builder.build()
    }

    fn on_event(&mut self, ui: &mut Ui<'_>, event: Self::Event) {
        match event {
            Event::NameChanged(text) => {
                self.input_text = text;
            }
            Event::RoleChanged { index, text } => {
                self.selected_role = text;
                println!("{index}");
            }
            Event::NewsletterChanged(checked) => {
                self.newsletter_opt_in = checked;
            }
            Event::ChannelEmailChanged(selected) => {
                if selected {
                    self.selected_channel = "Email".to_string();
                }
            }
            Event::ChannelSmsChanged(selected) => {
                if selected {
                    self.selected_channel = "SMS".to_string();
                }
            }
            Event::ChannelPushChanged(selected) => {
                if selected {
                    self.selected_channel = "Push".to_string();
                }
            }
            Event::VolumeChanged(value) => {
                self.volume = value;
                println!("volume: {:.2}", self.volume);
            }
            Event::ToggleClicked => {
                self.toggled = !self.toggled;
                self.update_panel_color(ui);
            }
        }

        self.refresh_toggle_button(ui);

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
        input_name: None,
        role_combo: None,
        newsletter_checkbox: None,
        channel_email: None,
        channel_sms: None,
        channel_push: None,
        volume_slider: None,
        toggle_button: None,
        main_panel: None,
    });
}
