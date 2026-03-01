//! Retained UI core.
//!
//! - `state`: owns widget nodes and persistent interaction data.
//! - `input`: applies per-frame input and emits high-level UI events.
//! - `paint`: renders retained nodes into the scene.
mod input;
mod node;
mod paint;
mod state;

pub(crate) use state::{
    RetainedState, UpsertCheckboxArgs, UpsertComboBoxArgs, UpsertRadioButtonArgs, UpsertSliderArgs,
    UpsertTextBoxArgs,
};
