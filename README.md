# Runo

`Runo` is an experimental GUI framework project built with `winit + wgpu + vello` in Rust.

## Features

- Retained-mode oriented UI architecture
- `Application` lifecycle with `build()` and `update()`
- Simple widget API (`button`, `label`)
- Basic layout containers (`vertical`, `horizontal`)
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
use runo_core::{Application, Ui, run};

struct MyApp {
    toggled: bool,
}

impl Application for MyApp {
    fn build(&mut self, ui: &mut Ui<'_>) {
        ui.vertical(|ui| {
            ui.label()
                .id("title")
                .text("runo example")
                .font_size(22)
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
        if ui.button_clicked("btnToggle") {
            self.toggled = !self.toggled;
            ui.set_button_text(
                "btnToggle",
                if self.toggled { "Toggle: ON" } else { "Toggle: OFF" },
            );
        }
    }
}

fn main() {
    run(MyApp { toggled: false });
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
