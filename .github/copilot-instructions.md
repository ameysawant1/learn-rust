# Copilot Instructions for Rust Learning Workspace

## Overview
This workspace is a collection of Rust learning projects, each in its own directory. The focus is on understanding Rust's core principles: ownership, concurrency, and systems-level safety. Projects are organized by topic and day, with each directory representing a distinct learning goal or experiment.

## Project Structure
- `day01_hello_rust/hello_cargo/` — Minimal Rust project using Cargo. Entry point: `src/main.rs`.
- `guessing_game/` — Another Cargo-based Rust project. Entry point: `src/main.rs`.
- `notes.md` — For personal notes and reflections (currently empty).
- `README.md` — High-level overview, learning goals, and progress tracking.

## Developer Workflows
- **Build & Run:**
  - Use `cargo build` and `cargo run` inside each project directory (e.g., `day01_hello_rust/hello_cargo/`, `guessing_game/`).
  - Example: `cd guessing_game && cargo run`
- **No custom build scripts or test suites** are present yet. All code is in `main.rs` files.
- **Rust Edition:** Projects use the `2024` edition (see `Cargo.toml`).

## Patterns & Conventions
- Each project is self-contained with its own `Cargo.toml` and `src/main.rs`.
- Code is simple and focused on learning, with minimal dependencies.
- All executable code starts in `fn main()`.
- No external crates or advanced Rust features are used yet.
- Directory names reflect the learning topic or day.

## Integration Points
- No external APIs, services, or cross-project dependencies.
- Future plans (see `README.md`) include async experiments with Tokio and custom crates for JSON parsing.

## Examples
- To add a new experiment, create a new directory with its own `Cargo.toml` and `src/main.rs`.
- To run any project: `cd <project_dir> && cargo run`

## Key Files
- `README.md` — Project philosophy, learning roadmap, and progress.
- `<project>/Cargo.toml` — Rust package manifest.
- `<project>/src/main.rs` — Main entry point for each experiment.

## Guidance for AI Agents
- Focus on clear, idiomatic Rust code for learning purposes.
- When adding new projects, follow the existing directory and file structure.
- Keep code and comments concise, emphasizing Rust concepts.
- Reference the `README.md` for context on learning goals and future directions.

---
If any section is unclear or missing, please provide feedback so instructions can be improved.
