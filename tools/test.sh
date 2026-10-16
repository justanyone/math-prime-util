#!/bin/sh
# Worker test command: `tools/test.sh <test file or filter>...` (the factory's `mw-test` calls this).
# Runs the crate's tests with the user's Rust toolchain on PATH (an ssh or worker
# shell has no ~/.cargo/bin). Arguments that name a file are mapped to cargo:
#   tests/compat_int.rs  -> --test compat_int        src/int/mod.rs -> --lib int
#   benches/int.rs       -> --bench int --no-run     anything else  -> passed through (a name filter)
# Limit: 900 s. Prints the last 60 lines.
export PATH="$HOME/.cargo/bin:$PATH"
cd "$(dirname "$0")/.." || exit 2
set --  "$@"
args=""
for a in "$@"; do
  case "$a" in
    tests/*.rs) n=${a#tests/}; n=${n%.rs}; args="$args --test $n" ;;
    src/*)      n=${a#src/}; n=${n%%/*}; n=${n%.rs}; [ "$n" = lib ] && args="$args --lib" || args="$args --lib $n" ;;
    benches/*.rs) n=${a#benches/}; n=${n%.rs}; args="$args --bench $n --no-run" ;;
    *)          args="$args $a" ;;
  esac
done
# shellcheck disable=SC2086
# --offline: worker hosts have no network in the sandbox; the maintainer runs `cargo fetch` on each host after a Cargo.lock change.
exec timeout 900 nice -n 10 cargo test --offline --all-features $args 2>&1 | tail -60
