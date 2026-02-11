# Runo

`Runo` is an experimental GUI framework project built with `winit + wgpu + vello` in Rust.

## Features

- Retained-mode oriented UI architecture
- `Application` lifecycle with `build()` and `update()`
- Widget API (`button`, `label`, `text_box`, `combo_box`, `div`)
- Basic layout containers (`vertical`, `horizontal`)
- Event-driven update model (`UiEvent`)
- Per-control enable/disable API (`set_*_enabled`, `enabled(...)`)
- Lightweight hook-like effect API (`use_effect`)

## Current Workspace

- `crates/runo_core`
  Core GUI framework implementation
- `crates/example`
  Example app that demonstrates how to build and update UI

## Quick Start

Requirements:

- Rust toolchain (`cargo`)

Run example app:

```bash
cargo run -p example
```

## Example Usage

```rust
use runo_core::{Application, RunOptions, Ui, UiEvent, run};

struct MyApp {
    toggled: bool,
    input_text: String,
    selected_role: String,
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
            ui.label()
                .id("title")
                .text("runo example")
                .font_size(22)
                .show();
            ui.text_box()
                .id("input.name")
                .width(320)
                .height(44)
                .font_size(18)
                .placeholder("Type here...")
                .show();
            ui.combo_box()
                .id("combo.role")
                .width(320)
                .height(44)
                .items(["Designer", "Engineer", "Manager"])
                .show();
            ui.button()
                .id("btnToggle")
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
                }
                UiEvent::TextBoxChanged { id, text } if id == "input.name" => {
                    self.input_text = text;
                }
                UiEvent::ComboBoxChanged { id, selected_text, .. } if id == "combo.role" => {
                    self.selected_role = selected_text;
                }
                _ => {}
            }
        }
    }
}

fn main() {
    run(MyApp {
        toggled: false,
        input_text: String::new(),
        selected_role: "Designer".to_string(),
    });
}
```

## Documentation

- Architecture: `crates/runo_core/docs/architecture.md`
- Universal GUI knowledge notes: `crates/runo_core/docs/README.md`
- Language-specific docs:
  - Japanese: `crates/runo_core/docs/ja/README.md`
  - English: `crates/runo_core/docs/en/README.md`

## License

MIT (`LICENSE`)
