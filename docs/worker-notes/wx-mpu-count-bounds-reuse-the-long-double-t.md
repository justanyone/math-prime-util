Port count bounds and approximation to the existing x87 LongDouble type.
Read PLAN sections 5–8 and preserve MPU formulas and rounding.
Remove precision skips and replay the existing oracle fixtures offline.
Run focused tests and the repository gate on this host if permitted.
- 2026-10-18T15:36:04Z launcher: no commit for 63 s; 94 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-count-bounds-reuse-the-long-double-t.md.
Reused the existing series, preserved the C Li NV conversions, and removed 41 precision skips. Host is wowbagger; requested remote gate is unavailable without prohibited access.
Verified exact x87 constants with decimal arithmetic; added ties-to-even, endpoint, repeated-call, and concurrent-call regression tests. Focused fixture test is compiling.
Offline clippy passed with all targets and features; the gate is building MSRV. Focused compatibility tests are still running. STATUS already marks all three functions complete.
MSRV build passed offline. Focused tests and the full gate are compiling; no test failure has been reported yet.
Focused compat_count finished: 12 passed, no skips for count bounds or approximation. The new binary64 rounding unit test also passed in the gate.
All focused checks passed. Waiting for the full offline gate; no further source changes are needed.
The full gate passed count compatibility and continues through integration tests. Diff whitespace checks passed; the launcher has snapshotted the source changes.
The gate is waiting on the existing forsquarefreeint integration test, which has exceeded 60 seconds. No failures; source and focused regressions are complete.
About five minutes remain. Full gate is still in the unrelated forsquarefreeint test; running offline warning-free documentation separately to record that check.
Offline all-feature documentation built with warnings denied. Running the count doctests while the full gate continues on forsquarefreeint.
All 11 matching count doctests passed. Documentation, fmt, clippy and MSRV passed offline; full gate is still waiting on forsquarefreeint.
