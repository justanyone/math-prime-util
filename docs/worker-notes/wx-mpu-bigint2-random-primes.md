Implement bigint random APIs using the shared seeded random stream and existing bigint primality.
Read PLAN sections 5 through 8 before editing code. Preserve native signatures.
Add bigint_random oracle generation, fixture replay, and property checks. Run offline checks.
Large factor sampling may be impractical in pure Perl; record measured limits and blockers here.
- 2026-10-19T15:42:59Z launcher: no commit for 61 s; 91 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-bigint2-random-primes.md.

Read PP sampling and Gordon algorithms. STATUS already ticks all twelve native names; no status edits needed. Host is wowbagger, not requested aragog or wonko; only local offline checks are possible.
Implemented eleven bigint APIs, including Gordon strong primes and Kalai factored sampling. Local oracle bootstrap is absent; checking vendored PP without network next.
Added all twelve APIs and fixture/property files. First focused offline test is running; oracle uses vendored MPU with MPU_NO_XS=1 because local::lib is unavailable.
First focused run found a fixture generator array reference encoding issue; fixed the generator to flatten factors and regenerated, never edited TSV. Added missing test module docs.
About five minutes remain. Focused replay exposed a PP safe-prime oracle error. Investigating the oracle exception; lint gate is running after formatting and fixture fixes.
All four focused tests pass, including every generated row. Documented MPU075_PP_SAFE_INTERNAL. Gate restarted after correcting clippy conversion and comparison findings.
Expanded native comparisons across 64-bit magnitudes and added prime-factor checks for 65-bit semiprimes. cargo-llvm-cov is absent, so coverage measurement is blocked offline; no package installation attempted.
