# Copilot Instructions for guessing_game

## Project Overview
This is a simple Rust project named `guessing_game`. The main entry point is `src/main.rs`, currently containing a basic `Hello, world!` example. The project is managed using Cargo, with configuration in `Cargo.toml`.

## Architecture & Structure
- **Single binary crate**: All logic resides in `src/main.rs`.
- **No library crate or modules**: The codebase is flat and minimal.
- **No external dependencies**: The `[dependencies]` section in `Cargo.toml` is empty.

## Developer Workflows
- **Build**: Run `cargo build` to compile the project.
- **Run**: Execute `cargo run` to build and run the binary.
- **Test**: No tests are present yet. Add tests in `src/main.rs` or create a `tests/` directory for integration tests.
- **Debug**: Use `cargo run` or `cargo build` with Rust debugging tools (e.g., `rust-gdb`).

## Conventions & Patterns
- **Edition**: Uses Rust 2024 edition (see `Cargo.toml`).
- **Entrypoint**: The `main` function in `src/main.rs` is the starting point.
- **No custom build scripts**: No `build.rs` or custom build logic.
- **No project-specific patterns**: Follows standard Rust binary crate conventions.

## Integration Points
- **Cargo**: All builds and runs are managed via Cargo.
- **No external services or APIs**: The project is self-contained.

## Key Files
- `src/main.rs`: Main program logic.
- `Cargo.toml`: Project metadata and dependencies.

## Example Workflow
```bash
# Build the project
cargo build

# Run the project
cargo run
```

## Guidance for AI Agents
- Focus on `src/main.rs` for all logic and entrypoint changes.
- If adding features, follow Rust idioms and keep code in `src/main.rs` unless refactoring to modules is required.
- If introducing dependencies, update `Cargo.toml` accordingly.
- If adding tests, use Rust's built-in test framework and consider creating a `tests/` directory for integration tests.

---
If any section is unclear or incomplete, please provide feedback to improve these instructions.