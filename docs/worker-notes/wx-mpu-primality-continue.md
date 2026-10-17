# wx-mpu-primality continue notes
- Goal: finish primality port (is_prime, Perrin, Catalan, Frobenius variants, Chen, delicate, safe, mersenne, benches) per task text.
- Plan: read current src/primality/mod.rs, add missing functions following existing pattern, add compat/prop tests, run tools/test.sh filters, then gate.
- miller_rabin_random stays unimplemented (todo in STATUS.md, random module missing).
- Added is_prime (wrapper over is_prob_prime, util.c) and compat test. Not yet done: perrin, catalan, frobenius x3, chen, delicate (no factoring in crate for chen/catalan), miller_rabin_random (random module absent).
- is_prime_oracle passes (tools/test.sh is_prime_oracle). Stopping at time limit. Remaining: is_perrin (needs calc_perrin_sig + tables), is_catalan (needs segmented sieve), frobenius x3 (not read), is_chen_prime (needs factoring; crate has none; fixture has 4.6e18), is_delicate_prime, miller_rabin_random (random module absent, todo), benches, prop tests for new fns, full gate not run.
