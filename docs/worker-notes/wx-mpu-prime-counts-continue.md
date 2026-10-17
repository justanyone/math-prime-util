# wx-mpu-prime-counts continue notes
- Goal: finish count family (LMO, nth_prime, chebyshev, sum_primes), tests, gate green.
- Plan: read src/count/mod.rs, implement missing functions, add tests, run focused tests.
- Added nth_prime (exact bsearch), chebyshev_theta/psi (f64), compat tests. Not yet: prime_count_lower/upper/approx, nth_prime_lower/upper/approx (need Axler/RiemannR ports), LMO, prop tests, bench.
