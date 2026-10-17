# wx-mpu-primality continue notes
- Goal: finish primality port (is_prime, Perrin, Catalan, Frobenius variants, Chen, delicate, safe, mersenne, benches) per task text.
- Plan: read current src/primality/mod.rs, add missing functions following existing pattern, add compat/prop tests, run tools/test.sh filters, then gate.
- miller_rabin_random stays unimplemented (todo in STATUS.md, random module missing).
- Added is_prime (wrapper over is_prob_prime, util.c) and compat test. Not yet done: perrin, catalan, frobenius x3, chen, delicate (no factoring in crate for chen/catalan), miller_rabin_random (random module absent).
