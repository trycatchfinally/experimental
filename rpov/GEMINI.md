# Gemini Assistant Instructions

This file provides instructions for the Gemini assistant to follow when working in this project.

## Project Overview

This is a Rust project that implements a ray tracer. The goal is to create a simple, yet powerful ray tracing engine that can generate high-quality images.

## File Structure

* `src/`: Contains the main source code for the ray tracer.
* `tests/`: Contains integration tests.
* `tests/features/`: Contains Gherkin feature files for acceptance tests.
* `target/`: Contains the compiled output. This directory should be ignored.

## Tooling and Commands

* **Build**: `cargo build`
* **Run**: `cargo run`
* **Test**: `cargo test`
* **Lint**: `cargo clippy`
* **Format**: `cargo fmt`

## Coding Style

* Follow the official Rust style guide.
* Use snake_case for variable and function names.
* Use CamelCase for struct and enum names.
* Add comments to explain complex logic.
* Don't add comments that duplicate what the code says.
* Do not write organizational or comments that summarize the code.
* Do not write comments like "// Implement Neg for Tuple".
* Prioritize code correctness and clarity. Speed and efficiency are secondary priorities unless otherwise specified.
* Never create files with `mod.rs` paths - prefer `src/some_module.rs` instead of `src/some_module/mod.rs`.

## Globbing

* Ignore the `target/` directory.
* Ignore `*.lock` files.
