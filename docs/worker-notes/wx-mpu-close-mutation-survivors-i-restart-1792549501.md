# Notes: close mutation survivors (lucky, ramanujan-upper, almost-prime)
- Plan: read the 4 files at the survivor lines, check each bound under the mutation, add tests/mutants_estimates.rs killing each survivor.
- Fix only where a bound is actually wrong; otherwise document as non-tight.
- 21:25 tests/mutants_estimates.rs already exists on branch (197 lines, 11 tests). Running it; Rust axler_d_tail constants match C ramanujan_primes.c:68-75.
- 2026-10-21T02:26:44Z launcher: no commit for 61 s; 91 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-close-mutation-survivors-i-restart-1792549501.md.
- 21:26 Ran lib tests filtered 'estimate': all pass. Not re-run cargo-mutants (no time). harmreal:763 '<'->'==' looks equivalent (terms < 1e-44 change f64 sum by nothing). No source edits made. Remaining: cargo-mutants confirmation and killing check per survivor.
