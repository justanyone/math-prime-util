# wx MPU close mutation survivors (lucky, ramanujan-upper, almost-prime)
- Plan: read the 4 named functions, decide per survivor whether bound is wrong or non-tight.
- Add killing tests in new tests/mutants_estimates.rs only; no existing test edits.
- Fix only if a bound is actually wrong. Re-run cargo-mutants on the 4 files if time allows.
- Wrote tests/mutants_estimates.rs (brute-force almost-prime, lucky sandwich, ramanujan >= p_n, harmreal). Next: run it.
