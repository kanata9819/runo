//! Bridge layer from builder args to retained upsert calls.
//!
//! Each `show_*` module maps user-facing builder parameters into concrete
//! retained node snapshots for the current frame.
pub(crate) mod button;
pub(crate) mod checkbox;
pub(crate) mod combo_box;
pub(crate) mod div;
pub(crate) mod label;
pub(crate) mod radio_button;
pub(crate) mod slider;
pub(crate) mod terminal_view;
pub(crate) mod text_box;
