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

* **Build**: `cargo build --all-targets`
* **Check**: `cargo check --all-targets`
* **Run**: `cargo run`
* **Test**: `cargo test`
* **Lint**: `cargo clippy`
* **Format**: `cargo fmt`

## Coding Style

* Use crate::floats::Float instead of f32 or f64.
* Follow the official Rust style guide.
* Do not prefix function arguments with underscore.
* Use snake_case for variable and function names.
* Use CamelCase for struct and enum names.
* Add comments to explain complex logic.
* Don't add comments that duplicate what the code says.
* Do not write organizational or comments that summarize the code.
* Do not write comments like "// Implement Neg for Tuple".
* Prioritize code correctness and clarity. Speed and efficiency are secondary priorities unless otherwise specified.
* Never create files with `mod.rs` paths - prefer `src/some_module.rs` instead of `src/some_module/mod.rs`.
* Ignore all "warning: unused import" messages and do not attempt to fix them.

## Globbing

* Ignore the `target/` directory.
* Ignore `*.lock` files.

## General guidance for automation

* Never remove files I created. Do not suggest or try to run the 'rm' command.
* It's okay to implement structs and constructors.
* Always ask before implementing non-trivial functionality.
* It's okay to add unit tests and statements in unit tests that don't compile or don't pass.
* Always check to see if a file exists before creating it.
* Never suggest `git` operations.
