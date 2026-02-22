# Guidelines for AI Agents

## Core Rules

- Do not use unstable features.
- Write tests for public code.
- Always run `cargo fmt` and `cargo clippy` after making code changes.
- Minimize the use of `unsafe` blocks. Whenever `unsafe` is used for FFI or OS APIs, you MUST include a `// SAFETY:` comment explaining why the operation is sound and safe.
- Add platform-specific dependencies strictly under the corresponding `[target.'cfg(target_os = "...")'.dependencies]` section in `Cargo.toml`.
- Ensure changes in common modules (e.g., `src/error.rs`, `src/cli.rs`) do not break compilation for other target operating systems. Use `#[cfg(target_os = "...")]` attributes appropriately.
- Map platform-specific errors cleanly to the unified `WasteError` enum, providing sufficient context. Do not expose raw OS errors to the user.

## Styles

- Write all code, comments, and documentation in English.
- Add documentation comments to the code you write, unless it is self-explanatory.

## Commands

- `cargo run -- <PATH>`: Run the `waste` CLI to move a file or directory to the trash.
- `cargo check`: Quickly check the code for compilation errors.
- `cargo test`: Run all tests in the project.
- `cargo fmt`: Format the codebase according to Rust style guidelines.
- `cargo clippy`: Run the linter to catch common mistakes and improve code quality.
