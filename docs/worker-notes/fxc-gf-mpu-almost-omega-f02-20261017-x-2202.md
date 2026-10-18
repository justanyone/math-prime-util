# fxc-gf-mpu-almost-omega-f02-20261017-x-2202

Resolve merge conflicts with origin/dev-001 in src/families/mod.rs, tests/compat_families.rs, and tests/prop_families.rs, keeping both sides. Then fix every Open reviewer finding with a regression test and run focused tests.

Why: the branch does not merge cleanly; the project gate requires a clean merge and passing tests without dropping either side's intent.

Next: read PLAN.md sections 5–8, conflict markers, and reviewer notes, then edit.
Read PLAN §§5–8 and the three conflicts. HEAD exports almost_prime_count_upper, nth_almost_prime, approx, and lower. origin/dev-001 exports nth_almost_prime_upper, omega_primes, omega_prime_count, and nth_omega_prime. No separate reviewer file; the Open finding is the merge. Keeping both sides and closing the split braces.
Resolved all three files (both export lists and both test modules). Formatting, then mw-test on compat_families and prop_families.
- 2026-10-18T03:04:12Z launcher: no commit for 61 s; 91 s elapsed. Files changed so far: README.md, STATUS.md, benches/arith.rs, benches/families.rs, benches/iter.rs, benches/modular.rs, benches/primality.rs, docs/worker-notes/fxc-gf-mpu-almost-omega-f02-20261017-x-2202.md, src/arith/chinese.rs, src/arith/exp_mangoldt.rs, src/arith/kronecker.rs, src/arith/liouville.rs.
