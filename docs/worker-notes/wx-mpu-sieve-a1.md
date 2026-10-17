# MPU-SIEVE worker notes

Will port MPU 0.75 sieve.c, cache.c, and sieve_cluster.c into src/sieve/.
Public API: primes, next_prime, prev_prime, sieve_range, sieve_prime_cluster, sum_primes, prime_precalc, prime_memfree.
Shared prime cache uses OnceLock/Mutex and is Send + Sync. No unsafe. No new Cargo.toml deps.
Cite C file and function in each public doc comment. Match existing int/modular test and bench patterns.
Next: read PLAN.md sections 5-8, crate patterns, and the C sieve entry points, then implement.
- 2026-10-16T23:57:15Z launcher: no commit for 61 s; 91 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-sieve-a1.md.
- 2026-10-16T23:58:15Z launcher: no commit for 60 s; 151 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-sieve-a1.md.
- 2026-10-16T23:59:15Z launcher: no commit for 60 s; 211 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-sieve-a1.md.
- 2026-10-17T00:00:17Z launcher: no commit for 61 s; 272 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-sieve-a1.md.
- 2026-10-17T00:01:17Z launcher: no commit for 60 s; 332 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-sieve-a1.md.
- 2026-10-17T00:02:17Z launcher: no commit for 60 s; 392 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-sieve-a1.md.
- 2026-10-17T00:03:18Z launcher: no commit for 61 s; 454 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-sieve-a1.md.
- 2026-10-17T00:04:18Z launcher: no commit for 60 s; 514 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-sieve-a1.md.
- 2026-10-17T00:05:18Z launcher: no commit for 60 s; 574 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-sieve-a1.md.
- 2026-10-17T00:06:19Z launcher: no commit for 61 s; 635 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-sieve-a1.md.
- 2026-10-17T00:07:19Z launcher: no commit for 60 s; 695 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-sieve-a1.md.
