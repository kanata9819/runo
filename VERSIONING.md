# Versioning Policy

This project uses Semantic Versioning (`MAJOR.MINOR.PATCH`) for published crates.

## Scope

- Primary scope: `crates/runo_core`.
- `crates/example` is not treated as a stable public API.

## SemVer Rules

1. `PATCH` (`0.1.x -> 0.1.y`)
- Bug fixes and internal refactors.
- No intentional public API break.

2. `MINOR` (`0.x -> 0.y`)
- New features, new APIs, non-breaking behavior changes.
- In `0.y.z`, breaking changes are still possible, but must be explicitly documented.

3. `MAJOR` (`1.x -> 2.0`)
- Breaking changes to stable public API.

## Breaking Change Rules

A change is considered breaking if it can require downstream user code changes, including:

- Public API removal or rename.
- Function signature/type changes.
- Behavioral changes that alter documented outcomes.
- Event payload/meaning changes.

When a breaking change is introduced:

1. Mark it in `CHANGELOG.md` under a `Breaking` or `Changed` entry with explicit migration notes.
2. Provide a migration snippet (before/after) in release notes or docs.
3. Prefer deprecation-first when practical:
- Add replacement API.
- Keep old API for at least one release cycle (when feasible).
- Remove in the next planned breaking release.

## Release Checklist (Minimum)

1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace --all-targets --all-features -- -D warnings`
3. `cargo test --workspace`
4. `cargo package -p runo_core`
5. `cargo publish --dry-run -p runo_core`
6. Update `CHANGELOG.md` and move relevant entries from `Unreleased` to a tagged version.
