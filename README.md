# Runo

`Runo` is an experimental GUI framework project built with `winit + wgpu + vello` in Rust.

## Features

- Retained-mode oriented UI architecture
- `RunoApplication` lifecycle with `build()` and `update()`
- Widget API (`button`, `label`, `text_box`, `combo_box`, `checkbox`, `radio_button`, `slider`, `div`)
- Basic layout containers (`vertical`, `horizontal`)
- Event-driven update model (`UiEvent`)
- State API (`set_text`, `set_value`, `set_enabled`, `combo_box().set_items(...)` etc.)
- Built-in palette presets and grouped color APIs (`Gray::gray_50()`, `Blue::blue_500()`, `Semantic::success()`)
- Lightweight hook-like effect API (`use_effect`)

## Current Workspace

- `crates/runo`
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

Run CI-equivalent local checks:

```bash
just ci
```

## Example Usage

```rust
use runo::{RunoApplication, Ui, UiEvent, run};

struct MyApp {
    count: u32,
}

impl RunoApplication for MyApp {
    fn build(&mut self, ui: &mut Ui<'_>) {
        ui.widgets()
            .button()
            .id("counter")
            .width(220)
            .height(56)
            .text("Clicked: 0")
            .show();
    }

    fn update(&mut self, ui: &mut Ui<'_>) {
        for event in ui.events().drain_events() {
            if let UiEvent::ButtonClicked { id } = event {
                if id == "counter" {
                    self.count += 1;
                    ui.state()
                        .button()
                        .set_text("counter", format!("Clicked: {}", self.count));
                }
            }
        }
    }
}

fn main() {
    run(MyApp { count: 0 });
}
```

## Documentation

- Architecture: `crates/runo/docs/architecture.md`
- Universal GUI knowledge notes: `crates/runo/docs/README.md`
- Changelog: `CHANGELOG.md`
- Versioning policy: `VERSIONING.md`
- Language-specific docs:
  - Japanese: `crates/runo/docs/ja/README.md`
  - English: `crates/runo/docs/en/README.md`

## License

MIT (`LICENSE`)
