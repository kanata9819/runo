# GUI Library Knowledge Notes (English)

This section contains implementation-agnostic knowledge for building GUI libraries.

## Current `runo_core` implementation notes

1. Built-in widgets
   `button`, `label`, `text_box`, `combo_box`, `checkbox`, `radio_button`, `slider`, `div`
2. Event model
   `UiEvent` (`ButtonClicked`, `CheckboxChanged`, `RadioButtonChanged`, `SliderChanged`, `TextBoxChanged`, `ComboBoxChanged`)
3. Enable/disable API
   `ui.state().*().set_enabled(...)` and per-widget `enabled(...)`

## Documents

1. `architecture.md`
   Current architecture of `runo_core`
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
