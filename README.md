# Runo

`Runo` is an experimental GUI framework project built with `winit + wgpu + vello` in Rust.

## Features

- Retained-mode oriented UI architecture
- `RunoApplication` lifecycle with `build()` and `update()`
- Widget API (`button`, `label`, `text_box`, `combo_box`, `checkbox`, `radio_button`, `slider`, `div`)
- Basic layout containers (`vertical`, `horizontal`)
- Event-driven update model (`UiEvent`, `ActionBindings`, `EventBindings`)
- State API (`set_text`, `set_value`, `set_enabled`, `combo_box().set_items(...)` etc.)
- Handle-based control API (`ButtonHandle`, `CheckboxHandle`, ... + optional handle ext in `prelude`)
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
use runo::{ButtonHandle, EventBindings, RunoApplication, Ui, run};

#[derive(Clone, Copy)]
enum AppEvent {
    CounterClicked,
}

struct MyApp {
    count: u32,
    counter: Option<ButtonHandle>,
    bindings: EventBindings<AppEvent>,
}

impl RunoApplication for MyApp {
    fn build(&mut self, ui: &mut Ui<'_>) {
        let counter = ui
            .widgets()
            .button()
            .id("counter")
            .width(220)
            .height(56)
            .text(format!("Clicked: {}", self.count))
            .show();
        self.bindings = EventBindings::builder()
            .button(counter.clone(), AppEvent::CounterClicked)
            .build();
        self.counter = Some(counter);
    }

    fn update(&mut self, ui: &mut Ui<'_>) {
        for event in ui.drain_bound_events(&self.bindings) {
            match event {
                AppEvent::CounterClicked => {
                    self.count += 1;
                    if let Some(counter) = &self.counter {
                        counter.set_text(ui, format!("Clicked: {}", self.count));
                    }
                }
            }
        }
    }
}

fn main() {
    run(MyApp {
        count: 0,
        counter: None,
        bindings: EventBindings::new(),
    });
}
```

## Documentation

- Architecture: `crates/runo/docs/architecture.md`
- Universal GUI knowledge notes:
  - Japanese: `crates/runo/docs/ja/README.md`
  - English: `crates/runo/docs/en/README.md`
- Changelog: `CHANGELOG.md`
- Versioning policy: `VERSIONING.md`

## License

MIT (`LICENSE`)
