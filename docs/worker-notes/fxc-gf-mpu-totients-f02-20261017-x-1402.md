# fxc notes: resolve merge conflict tests/prop_arith.rs with origin/dev-001
- Plan: find conflict markers, keep both sides' intent, run mw-test on prop_arith.
- Resolved all 4 markers in tests/prop_arith.rs (kept both sides as separate tests). Running mw-test.
- mw-test broken here (passes --collect-only to cargo); using tools/test.sh prop_arith
