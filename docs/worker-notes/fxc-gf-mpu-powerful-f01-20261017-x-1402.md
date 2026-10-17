# fxc notes: resolve merge conflicts with origin/dev-001
- Plan: resolve conflict markers in benches/families.rs and tests/prop_families.rs keeping both sides.
- Then run focused tests via mw-test, then commit (launcher commits).
- Benches import list reflowed by hand. cargo/rustfmt and mw-test not runnable here (mw-test passes unsupported --collect-only). Verified by reading only.
