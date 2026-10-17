# pc1-mpu-ramanujan-f01-t0958

Continue the PARTIAL Ramanujan prime-count port. The lower and approx functions exist, but the two `u64::MAX` cases are skipped with comments instead of computed. I will read PLAN.md §§5–8 and the current functions, match the C reference, replace the skips with real results, add tests that execute those lines, and run `tools/test.sh` plus `tools/gate.sh`.

- Minute 0: notes written; next is PLAN sections 5–8 and the two family sources.
