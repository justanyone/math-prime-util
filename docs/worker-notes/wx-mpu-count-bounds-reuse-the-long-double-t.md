Port count bounds and approximation to the existing x87 LongDouble type.
Read PLAN sections 5–8 and preserve MPU formulas and rounding.
Remove precision skips and replay the existing oracle fixtures offline.
Run focused tests and the repository gate on this host if permitted.
- 2026-10-18T15:36:04Z launcher: no commit for 63 s; 94 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-count-bounds-reuse-the-long-double-t.md.
Reused the existing series, preserved the C Li NV conversions, and removed 41 precision skips. Host is wowbagger; requested remote gate is unavailable without prohibited access.
Verified exact x87 constants with decimal arithmetic; added ties-to-even, endpoint, repeated-call, and concurrent-call regression tests. Focused fixture test is compiling.
Offline clippy passed with all targets and features; the gate is building MSRV. Focused compatibility tests are still running. STATUS already marks all three functions complete.
