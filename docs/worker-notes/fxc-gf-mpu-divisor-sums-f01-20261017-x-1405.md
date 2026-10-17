# fxc notes (merge conflict resolution)
- Plan: resolve conflict markers in src/arith/mod.rs and tests/prop_arith.rs keeping both sides.
- Then run focused tests with mw-test and commit-ready tree.
- mw-test fails here (passes --collect-only to cargo test); falling back to tools/test.sh prop_arith.
