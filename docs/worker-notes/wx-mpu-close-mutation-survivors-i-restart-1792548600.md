# Notes: close mutation survivors (lucky, ramanujan-upper, almost-prime)
- Plan: read the 4 source functions, check each bound at boundary inputs, write tests/mutants_estimates.rs.
- Fix only where a bound is actually wrong; otherwise record the survivor as non-tight.
- Survivors sit in private fns; integration tests reach only public API. Writing boundary bound tests via public fns.
- Wrote tests/mutants_estimates.rs (bound sandwich tests). Running focused test next.
- BLOCKER: lib unit tests do not compile on this checkout: macros public_estimate_tests / private_estimate_tests are used in src/families/{lucky,nth_ramanujan_prime_upper,nth_almost_prime}.rs and src/real/mod.rs but defined nowhere. Not fixed (outside task paths). New test file not run. cargo-mutants not re-run.
