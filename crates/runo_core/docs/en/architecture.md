# Runo Core Architecture

This document explains the internal structure of `runo_core`.

## 1. Core stack

`runo_core` uses:

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
4. `ComboBox` (dropdown-style select)

## 3. Application lifecycle

```rust
pub trait Application {
    fn build(&mut self, _ui: &mut Ui<'_>) {}
    fn update(&mut self, _ui: &mut Ui<'_>) {}
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
6. Run `Application::update()`
7. Render retained widgets
8. Submit/present via GPU

## 5. Module map

1. `app/`: runtime orchestration
   `mod.rs`, `runner.rs`, `events.rs`, `frame.rs`, `gpu.rs`
   `frame.rs` separates `surface_size`, `compose_frame`, and `submit_frame`
2. `retained/`: retained UI core
   `node.rs`, `state.rs`, `input.rs`, `paint.rs`
   includes interaction state, enabled/disabled state, and dropdown handling
3. `ui.rs`: user-facing UI API
   `button`, `label`, `text_box`, `combo_box`
   `drain_events`, `next_event`
   `set_button_enabled`, `set_text_box_enabled`, `set_combo_box_enabled`, `set_label_enabled`
4. `widget/`: builders (`button`, `label`, `text_box`, `combo_box`) and text helpers
5. `input.rs`, `layout/mod.rs`, `hooks/effect.rs`, `font.rs`: support modules
