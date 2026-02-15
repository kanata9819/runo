# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project follows Semantic Versioning.

## [Unreleased]

### Added
- CI workflow for `fmt`, `clippy`, and `test`.
- Integration test scaffold for public API (`crates/runo_core/tests/public_api.rs`).
- Expanded unit tests for state/input/cache/widget helpers.

### Changed
- Refactored retained upsert APIs to use typed argument structs.
- Added publish metadata to crate manifests.

### Fixed
- Corrected text box caret vertical movement test expectation for shorter line clamping.

## [0.1.0] - 2026-02-15

### Added
- Initial experimental retained-mode GUI framework structure.
- Core widget set: `button`, `label`, `text_box`, `combo_box`, `checkbox`, `radio_button`, `slider`, `div`.
- Event-driven update model and UI state APIs.
