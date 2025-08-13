#!/bin/bash

set -e

run_checks() {
  echo "Running: cargo build $@"
  cargo build --all-targets "$@"

  echo "Running: cargo test $@"
  cargo test --all-targets "$@"

  echo "Running: cargo clippy $@ -- -Dwarnings"
  cargo clippy --all-targets "$@" -- -Dwarnings
}

run_checks
run_checks --features=f64
echo ""
echo "========================"
echo "🎉 All checks passed! 🎉"
echo "========================"
