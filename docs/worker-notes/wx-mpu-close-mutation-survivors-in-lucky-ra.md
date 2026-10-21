# wx MPU close mutation survivors (lucky, ramanujan-upper, almost-prime)
- Plan: read the 4 named functions, decide per survivor whether bound is wrong or non-tight.
- Add killing tests in new tests/mutants_estimates.rs only; no existing test edits.
- Fix only if a bound is actually wrong. Re-run cargo-mutants on the 4 files if time allows.
- Wrote tests/mutants_estimates.rs (brute-force almost-prime, lucky sandwich, ramanujan >= p_n, harmreal). Next: run it.
- 2026-10-21T01:08:29Z launcher: no commit for 60 s; 121 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-close-mutation-survivors-in-lucky-ra.md, tests/mutants_estimates.rs.
