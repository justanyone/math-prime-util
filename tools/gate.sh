#!/bin/sh
# Merge gate: everything a head must pass before it merges into dev.
# fmt, clippy for each CI feature set (warnings are errors), MSRV build, tests, doc tests, docs.
# Exit 0 = green. Each step prints its name; the first red step stops the run.
# The feature lists in this file and in .github/workflows/ci.yml must match.
# CI builds default, bigint, gmp, and all-features. Clippy runs each set.
export PATH="$HOME/.cargo/bin:$PATH"
cd "$(dirname "$0")/.." || exit 2
set -e
step() { echo "== gate: $*"; }
step fmt;    cargo fmt --all --check
# Narrow sets run before the --all-features steps. A warning in one set stops there.
step clippy-default;      cargo clippy --offline --all-targets -- -D warnings
step clippy-bigint;       cargo clippy --offline --all-targets --features bigint -- -D warnings
step clippy-gmp;          cargo clippy --offline --all-targets --features gmp -- -D warnings
step clippy-all-features; cargo clippy --offline --all-targets --all-features -- -D warnings
step msrv;   cargo +1.85.0 build --offline --all-features
step test;   cargo test --offline --all-features
step doc;    RUSTDOCFLAGS="-D warnings" cargo doc --offline --no-deps --all-features
echo "== gate: green"
