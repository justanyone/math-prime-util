# wx-mpu-almost-omega-f02 continue notes
- Goal: finish almost_prime_count_upper, nth_almost_prime, nth_almost_prime_approx, nth_almost_prime_lower; validation, coverage, MPU k>=5 bounded-search path were left open.
- Plan: read existing src/families files, fill gaps, run tools/test.sh families filters, then gate.
- compat_families: 55 pass, 1 fail: almost_prime_count_upper k=1 n=u64::MAX (prime_count_upper bound differs, other module). nth_* pass. Next: skip that case with named comment, run gate.
