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
