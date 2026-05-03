# Runo Studio

`runo-studio` is the visual builder for Runo applications.

The first MVP is intentionally small:

1. Add `Button`, `Label`, and `TextBox` controls from a palette
2. Select controls in an outline
3. Edit text, width, and height
4. Preview the current document
5. Generate Rust code for the document

## Architecture

1. `document.rs`: UI document model independent from rendering
2. `codegen.rs`: Rust code generation from the document model
3. `studio.rs`: Runo-powered editor shell

This separation keeps the visual editor ready for later drag/drop placement,
save/load, undo/redo, and richer generated code.

## Run

```bash
cargo run -p runo-studio
```
