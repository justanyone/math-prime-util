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
- 2026-10-17T00:08:19Z launcher: no commit for 60 s; 755 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-sieve-a1.md.
19:09 Read PLAN 5-8, cache.c, sieve.h, cluster bounds, fixtures. Next: write src/sieve and tests.
- 2026-10-17T00:10:21Z launcher: no commit for 60 s; 876 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-sieve-a1.md.
- 2026-10-17T00:11:21Z launcher: no commit for 60 s; 936 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-sieve-a1.md.
- 2026-10-17T00:12:22Z launcher: no commit for 61 s; 998 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-sieve-a1.md.
- 2026-10-17T00:13:52Z launcher: no commit for 60 s; 1088 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-sieve-a1.md, src/sieve/mod.rs.
19:15 Wrote sieve module, compat, prop, bench. Running lib tests.
Lib tests: is_prime treated 5 and 7 as composite. Fixed the trial division. Re-run lib tests, then compat.
19:17 Compat and prop tests passed. Clippy clean. Running gate.
19:18 Gate printed == gate: green. Sieve API, cache, compat, prop, and bench are on disk. Done.
