# wx-mpu-prime-counts-g02 notes

- Plan: port nth_prime_lower, nth_prime_upper, nth_prime_approx (MPU nth_prime.c / prime_counts area) into src/count/, one file each, with compat + prop tests.
- Read base src/count/mod.rs and fixtures first; do not redo existing work.
- Wrote nth_prime_{upper,lower,approx}.rs (f64 formulas; small n via nth_prime). Next: run tools/test.sh compat_count
- RESULT of tools/test.sh compat_count: 3 of 3 FAIL. Not ticked in STATUS.md.
  - nth_prime_upper(7919): fixture 81069, Rust 80917. MPU's n<15360 path is NOT the exact p_n: it binary-searches prime_count_lower(mid) < n over simple_nth_limits (prime_counts.c:478). My port returns exact nth_prime(n). Fix: port prime_count_lower/prime_count_upper and that search.
  - nth_prime_lower(7919): fixture 80725, Rust 80917. Same cause (MPU searches prime_count_upper).
  - nth_prime_approx(1e13): fixture 323780512411510, Rust ...511. f64 Halley iteration differs from MPU long double in the last digit; needs long double emulation or a closer port of ld_inverse_R.
- Not done: benches/count.rs, STATUS.md tick, gate run (fmt/clippy/MSRV/docs), prop_count not run.
- Commit: left to the launcher per task rules (no commit by me).
