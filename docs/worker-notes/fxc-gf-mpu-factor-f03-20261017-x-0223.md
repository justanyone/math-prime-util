# fxc MPU-FACTOR-F03 merge resolution
- Goal: resolve conflict markers in benches/factor.rs, src/factor/mod.rs, tests/compat_factor.rs, tests/prop_factor.rs, keeping both sides.
- Plan: inspect each conflict hunk, merge both intents, run focused tests via mw-test, commit via launcher.
- Resolved all 4 conflicts by keeping both sides (HEAD bounded factor methods + origin predicates). Next: run mw-test on factor tests.
