# Rust Learning Workspace — Notes

## Project Structure
- Each experiment is a separate Cargo project in its own directory.
- Entry point for each: `src/main.rs`.

## Day 1: Hello Rust
- Minimal Rust program: prints `Hello, world!`.
- Focus: verifying toolchain, basic Cargo usage.

## Guessing Game
- Uses `rand` crate for random number generation.
- Prompts user to guess a number between 1 and 100.
- Provides hints: even/odd, <=50/>50.
- Reads user input, parses to `u32`, checks against target.
- Prints feedback: win, too high, too low, and reveals the number.
- Demonstrates basic I/O, string parsing, and control flow.

## Patterns & Conventions
- All logic in `fn main()` for simplicity.
- No modules, tests, or advanced features yet.
- Comments and code are minimal, focused on learning.

## Next Steps
- Add more experiments: ownership, borrowing, lifetimes, async (Tokio), custom crates.
- Expand code to use more Rust features and idioms.

---
These notes summarize the current codebase and learning focus. Update as new experiments are added.
