# fxc notes: resolve merge conflict in src/arith/mod.rs
- Plan: resolve conflict markers keeping both sides' module/re-export intent, then run focused tests.
- Resolved conflict in src/arith/mod.rs by keeping both module sets (qnr/is_qr/is_fundamental + ramanujan_sum/valuation). Next: build + focused test.
- Focused test tools/test.sh --lib arith: 17 passed. Merge resolved; no new behaviour so no new failing-before test. Left uncommitted for launcher.
