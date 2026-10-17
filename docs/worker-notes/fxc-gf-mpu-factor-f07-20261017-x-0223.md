# fxc-gf mpu-factor-f07 merge resolution (0223)
- Plan: resolve conflict markers in benches/factor.rs, src/factor/mod.rs, tests/compat_factor.rs, tests/prop_factor.rs keeping both sides.
- Then run focused tests with mw-test; add test for any behavior fix.
- Resolved all 4 conflicts by keeping both sides (single factor mod, combined bench, compat, prop). Next: run mw-test.
- Gate and cargo fmt/rustfmt need approval in this sandbox; not run. Tests passed before the manual formatting tweak in prop_factor.rs; rerunning test target. Left uncommitted (launcher commits).
