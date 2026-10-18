Continuing the partial prime-count implementation on this branch.
Compare the nth-prime bounds and approximation with MPU 0.75 and repair compatibility failures.
Finish count properties and benchmarks, then run focused tests and the required gate.
Keep STATUS limited to completed function ticks; do not mark the task READY.
Initial inspection: small-index bounds incorrectly use exact primes instead of MPU binary searches; checking existing real-number helpers before repair.
Ported the small-index inverse count searches, preserving MPU float coefficient promotion; next verify fixtures and numerical precision limits.
Focused compat rerun confirms small-index repairs; remaining failures are large long-double rounding and Perl bigint dispatch outside native bounds. Added inversion/approximation properties and Criterion bound workloads.
Oracle regeneration blocked: .perl-local/bin/cpanm and local::lib are absent; no network bootstrap attempted. Added regression tests and coverage across coefficient ranges.
All five count property tests pass and the benchmark target builds. A pre-existing unit expectation used the wrong 15360th prime; replaced it with the exact nth_prime result. Reverted the unregenerated generator change to keep fixtures synchronized.
