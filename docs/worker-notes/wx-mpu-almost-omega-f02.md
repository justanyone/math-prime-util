Port the four named almost-prime functions from MPU 0.75.
Check existing count and interpolation dependencies before choosing shared helpers.
Keep fixtures, tools, STATUS.md, and the crate root unchanged as required by the task.
Run focused offline tests and the gate; record any missing dependencies.
Dependency review: count bounds and inverse interpolation need private ports; semiprime APIs already exist.
- 2026-10-17T18:45:34Z launcher: no commit for 60 s; 122 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-almost-omega-f02.md.
Implemented bounds tables, reductions, count inversion, and the four public functions. Next add fixture and property coverage.
Added native oracle checks, property tests, concurrency checks, and benchmarks. Focused tests are compiling offline.
Focused tests and gate are waiting on the Cargo build lock. Next inspect failures once compilation finishes.
Corrected the prime approximation branch to use MPU real.c Halley inverse R. Exact nth search still lacks the C approximation-assisted optimization.
Added the C approximation-assisted nth search for k=3 and k=4 with tolerance 20000. Cargo commands still wait on the shared build lock.
Shared build lock has not cleared. Started the focused oracle tests in a worktree-local target directory to get independent offline validation.
- 2026-10-17T18:49:38Z launcher: no commit for 60 s; 366 s elapsed. Files changed so far: benches/families.rs, docs/worker-notes/wx-mpu-almost-omega-f02.md, src/families/almost_prime_count_upper.rs, src/families/mod.rs, src/families/nth_almost_prime.rs, src/families/nth_almost_prime_approx.rs, src/families/nth_almost_prime_lower.rs, tests/compat_families.rs, tests/prop_families.rs.
Offline validation remains running. No fixture or tool changes were made; all test cases outside the native u64 API are explicitly skipped.
About five minutes remain. Both Cargo target directories still have pending builds, so no validation result is available yet.
Formatting passes. Gate and focused tests remain pending behind Cargo compilation; native count recursion currently omits the C prime-count cache optimization.
Independent compilation found u32/u64 mismatches at is_almost_prime calls; fixed with lossless conversions and restarted focused tests.
Clippy reached the new code and rejected ungrouped numeric literals. Added separators without changing values; focused fixture tests are still pending.
- 2026-10-17T18:53:11Z launcher: no commit for 60 s; 579 s elapsed. Files changed so far: benches/families.rs, docs/worker-notes/wx-mpu-almost-omega-f02.md, src/families/almost_prime_count_upper.rs, src/families/mod.rs, src/families/nth_almost_prime.rs, src/families/nth_almost_prime_approx.rs, src/families/nth_almost_prime_lower.rs, tests/compat_families.rs, tests/prop_families.rs.
Added an explicit native-input skip for bigint fixture n. Final gate still pending; exact counting lacks the C prime-count cache and full k>=5 threshold optimization.
Added the C cache-size policy and a per-call prime-count cache to the recursive exact count. Approximation inversion uses the full Ridder search.
Near deadline: no green gate yet. The four public APIs and tests are written; validation and the C k>=5 bounded-search optimization remain to verify.
Gate found the existing benchmark function exceeded its line limit after additions. Moved the new benchmarks to a separate registered function.
- 2026-10-17T18:55:44Z launcher: no commit for 60 s; 732 s elapsed. Files changed so far: benches/families.rs, docs/worker-notes/wx-mpu-almost-omega-f02.md, src/families/almost_prime_count_upper.rs, src/families/mod.rs, src/families/nth_almost_prime.rs, src/families/nth_almost_prime_approx.rs, src/families/nth_almost_prime_lower.rs, tests/compat_families.rs, tests/prop_families.rs.
