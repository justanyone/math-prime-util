# fxc notes: resolve merge conflicts with origin/dev-001
- Plan: resolve conflict markers in benches/random.rs and src/random/mod.rs keeping both sides.
- Then run focused tests via mw-test and check compile.
- Resolved both conflicts (kept both module lists; bench merged). Next: compile + mw-test random tests.
- Fixed missing unhex helper in tests/compat_random.rs (dev-001 side). Running tools/test.sh random again.
