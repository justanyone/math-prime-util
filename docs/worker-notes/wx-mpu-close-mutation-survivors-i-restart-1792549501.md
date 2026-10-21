# Notes: close mutation survivors (lucky, ramanujan-upper, almost-prime)
- Plan: read the 4 files at the survivor lines, check each bound under the mutation, add tests/mutants_estimates.rs killing each survivor.
- Fix only where a bound is actually wrong; otherwise document as non-tight.
- 21:25 tests/mutants_estimates.rs already exists on branch (197 lines, 11 tests). Running it; Rust axler_d_tail constants match C ramanujan_primes.c:68-75.
