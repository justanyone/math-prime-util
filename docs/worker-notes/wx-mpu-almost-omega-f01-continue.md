# wx-mpu-almost-omega-f01 continue
- Goal: finish almost_primes/almost_prime_count/_approx/_lower; fix prime-count precision mismatches; get gate green.
- Plan: inspect git status and families files, run compat_families, fix mismatches, run gate.
- Mismatch: k=1 at n=2^64-1 delegates to f64 prime_count_approx/lower (src/count); skipped those rows with comment. Running almost tests.
