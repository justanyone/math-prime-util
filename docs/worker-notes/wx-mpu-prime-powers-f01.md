Port the four prime power functions from the native MPU algorithms.
Keep each public function in its own file and use the existing integer and prime APIs.
Use the committed families fixtures without changing tools or oracle data.
Add compatibility, algebraic, boundary, and benchmark coverage; run offline checks.
Read the native reference and existing test patterns; implemented all four functions with the C scan threshold and neighbor bounds.
Added all oracle tests, prime-root and neighbor laws, boundary and concurrent repeat-call tests, plus four benchmarks. Focused offline tests are running; corrected undef handling for previous prime powers.
Focused compatibility and property tests pass (22 tests). Three next-prime-power oracle results are 2^64; the test documents this native-API limit and checks None. Running the full offline gate next.
The gate found four missing benchmark semicolons under pedantic clippy. Fixed them and added a no-higher-powers merge-path assertion; rerunning the gate.
The second gate passed formatting, clippy, and the offline Rust 1.85 build. Full tests and docs are still running; git diff --check is clean.
- 2026-10-17T06:51:07Z launcher: no commit for 60 s; 212 s elapsed. Files changed so far: benches/families.rs, docs/worker-notes/wx-mpu-prime-powers-f01.md, src/families/is_prime_power.rs, src/families/mod.rs, src/families/next_prime_power.rs, src/families/prev_prime_power.rs, src/families/prime_powers.rs, tests/compat_families.rs, tests/prop_families.rs.
The full gate is still in existing compat_count tests; no failure has appeared. All task-specific tests pass and the code is ready while the gate completes.
Existing chebyshev, nth-prime, and prime-count oracle tests exceed 60 seconds. The gate remains active; no new code changes are needed.
