#!/bin/sh
# Merge gate: everything a head must pass before it merges into dev.
# fmt, clippy (warnings are errors), MSRV build, tests, doc tests, docs.
# Exit 0 = green. Each step prints its name; the first red step stops the run.
export PATH="$HOME/.cargo/bin:$PATH"
cd "$(dirname "$0")/.." || exit 2
set -e
step() { echo "== gate: $*"; }
step fmt;    cargo fmt --all --check
step clippy; cargo clippy --all-targets --all-features -- -D warnings
step msrv;   cargo +1.85.0 build --all-features
step test;   cargo test --all-features
step doc;    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
echo "== gate: green"
