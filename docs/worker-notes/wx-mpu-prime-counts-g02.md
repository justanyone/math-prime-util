# wx-mpu-prime-counts-g02 notes

- Plan: port nth_prime_lower, nth_prime_upper, nth_prime_approx (MPU nth_prime.c / prime_counts area) into src/count/, one file each, with compat + prop tests.
- Read base src/count/mod.rs and fixtures first; do not redo existing work.
- Wrote nth_prime_{upper,lower,approx}.rs (f64 formulas; small n via nth_prime). Next: run tools/test.sh compat_count
