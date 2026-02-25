# GUI Library Knowledge Notes (English)

This section contains implementation-agnostic knowledge for building GUI libraries.

## Current `runo` implementation notes

1. Built-in widgets
   `button`, `label`, `text_box`, `combo_box`, `checkbox`, `radio_button`, `slider`, `div`
2. Event model
   `UiEvent` (handle-based events) + `ActionBindings` / `EventBindings` / `EventBindingsBuilder`
3. Enable/disable API
   `ui.state().*().set_enabled(...)` and per-widget `enabled(...)`
4. Dynamic combo box items API
   `ui.state().combo_box().set_items(...)`
5. Handle API
   `ButtonHandle` and others provide `on_click` / `take_click` / `set_enabled` style operations
6. Optional handle extensions
   `prelude::*` exposes `Optional*HandleExt` helpers (`on_click` / `on_change` / `take_change`)
7. Color presets
   constants + grouped APIs like `Gray::gray_50()` and `Semantic::success()`

## Documents

1. `architecture.md`
   Current architecture of `runo`
2. `knowledge-fundamentals.md`
   Core concepts of GUI library design
3. `knowledge-event-input.md`
   Event loop and input model design
4. `knowledge-layout-widget.md`
   Layout and widget design principles
5. `knowledge-rendering-performance.md`
   Rendering pipeline and performance practices
6. `knowledge-testing-release.md`
   Testing strategy and release checklist

## Local quality checks

Use `just ci` to run the same checks as CI (`fmt --check`, strict `clippy`, workspace tests).
