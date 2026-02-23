# Runo Core Architecture

This document explains the internal structure of `runo`.

## 1. Core stack

`runo` uses:

1. `winit` for windowing and event loop
2. `wgpu` for GPU execution
3. `vello` for vector/text rendering

## 2. UI model

Runo is designed in a retained-mode style:

1. Widgets are persisted as nodes
2. Input updates node state
3. Persisted nodes are rendered each frame

Current built-in widgets:

1. `Button`
2. `Label`
3. `TextBox`
4. `ComboBox`
5. `Checkbox`
6. `RadioButton`
7. `Slider`

## 3. RunoApplication lifecycle

```rust
pub trait RunoApplication {
    fn build(&mut self, _ui: &mut Ui<'_>) {}
    fn update(&mut self, _ui: &mut Ui<'_>) {}
    fn options(&self) -> RunOptions {
        RunOptions::default()
    }
}
```

1. `build`: initial UI construction (typically once)
2. `update`: per-frame state updates

## 4. Frame flow

1. Receive OS events
2. Normalize input state
3. Start render
4. Draw background
5. Update retained widget interaction state (`hovered`, `pressed`, `focused`, `is_open`)
6. Run `RunoApplication::update()`
7. Render retained widgets
8. Submit/present via GPU

## 5. Module map

1. `app/`: runtime orchestration
   owns application lifecycle, window/surface setup, and frame submission boundaries
   `mod.rs`: public entry (`RunoApplication`, `run`)
   `runner.rs`: `AppRunner` state holder (window, renderer, input, effects, retained state)
   `events.rs`: OS event handling via `winit::ApplicationHandler`
   `frame.rs`: frame pipeline split (`surface_size`, `compose_frame`, `submit_frame`)
   `gpu.rs`: surface acquire/render/present and GPU error classification
2. `retained/`: retained UI core
   single source of truth for persistent widget nodes and interaction state
   `node.rs`: widget node structs (rect/state/flags per widget)
   `state/`: state mutation/query API split by concern (`core`, `button`, `checkbox`, `radio_button`, `slider`, `text_box`, `combo_box`)
   `input/mod.rs`: retained input pipeline entry
   `input/pointer.rs`: pointer-driven transitions (hover/press/drag/dropdown)
   `input/text_box.rs`: text editing, caret movement, scroll behavior
   `paint/mod.rs` + `paint/*.rs`: widget-specific rendering dispatch/implementation
3. `ui/`: user-facing API split by role
   high-level API consumed by applications
   `ui/mod.rs`: `Ui` root object and layout/effect helpers
   `ui/widgets.rs`: `ui.widgets()` builder entrypoints
   `ui/state.rs`: `ui.state()` mutation API (`set_text`, `set_value`, `combo_box().set_items`, etc.)
   `ui/events.rs`: `drain_events` / `next_event`
   `ui/show/*.rs`: adapter layer from builders to retained state upserts
4. `ui/colors`: constants + grouped APIs (`Gray::gray_50()`, `Blue::blue_500()`, `Semantic::success()`)
5. `widget/`: builders and response types (`button`, `label`, `checkbox`, `radio_button`, `slider`, `text_box`, `combo_box`)
6. Support modules
   `input.rs`: normalized per-frame input snapshot
   `layout/mod.rs`, `layout/stack.rs`, `layout/div.rs`: layout allocation and container flow
   `hooks/effect.rs`: effect dependency tracking and cleanup execution
   `cache/mod.rs`, `cache/text_layout.rs`: text layout caching
   `event.rs`: `UiEvent` definitions
   `font.rs`: default font discovery/loading
