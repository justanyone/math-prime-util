# MPU-RANDOM-PRIMES-F05

Port `vecsample` and `miller_rabin_random` into `src/random/` as their own files. Native `u64` only; bit sizes above 64 return `None`. Follow MPU 0.75 algorithms from `reference/mpu` and the existing module pattern. Add compat and property tests. Do not edit STATUS.md, lib.rs, tools, or fixtures.

- Start: notes written. Next: read PLAN §§5–8, existing `src/random`, and the two functions in the C/Perl sources.
- Found `src/random` already has the stream and prime samplers. `random.tsv` has 112 `vecsample` rows and no `miller_rabin_random` rows. Porting XS Fisher-Yates and `is_mr_random` next.
- Wrote `vecsample.rs`, `miller_rabin_random.rs`, compat and property tests, and two benches. Next: focused tests.
- Compat and property tests passed. Clippy is clean. Fixed `assert_is_empty`. Next: the gate script.
- 2026-10-17T06:59:55Z launcher: no commit for 61 s; 91 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-random-primes-f05-a1.md.
- 2026-10-17T07:00:55Z launcher: no commit for 60 s; 151 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-random-primes-f05-a1.md.
- 2026-10-17T07:01:55Z launcher: no commit for 60 s; 211 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-random-primes-f05-a1.md.
- 2026-10-17T07:02:57Z launcher: no commit for 61 s; 272 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-random-primes-f05-a1.md.
- 2026-10-17T07:03:57Z launcher: no commit for 60 s; 332 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-random-primes-f05-a1.md.
- 2026-10-17T07:04:57Z launcher: no commit for 60 s; 392 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-random-primes-f05-a1.md.
- 2026-10-17T07:05:58Z launcher: no commit for 61 s; 454 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-random-primes-f05-a1.md.
- 2026-10-17T07:09:30Z launcher: no commit for 60 s; 665 s elapsed. Files changed so far: benches/random.rs, docs/worker-notes/wx-mpu-random-primes-f05-a1.md, src/random/miller_rabin_random.rs, src/random/mod.rs, src/random/vecsample.rs, tests/compat_random.rs, tests/prop_random.rs.
- 2026-10-17T07:10:30Z launcher: no commit for 60 s; 725 s elapsed. Files changed so far: benches/random.rs, docs/worker-notes/wx-mpu-random-primes-f05-a1.md, src/random/miller_rabin_random.rs, src/random/mod.rs, src/random/vecsample.rs, tests/compat_random.rs, tests/prop_random.rs.
- 2026-10-17T07:11:32Z launcher: no commit for 61 s; 787 s elapsed. Files changed so far: benches/random.rs, docs/worker-notes/wx-mpu-random-primes-f05-a1.md, src/random/miller_rabin_random.rs, src/random/mod.rs, src/random/vecsample.rs, tests/compat_random.rs, tests/prop_random.rs.
- 2026-10-17T07:12:32Z launcher: no commit for 60 s; 847 s elapsed. Files changed so far: benches/random.rs, docs/worker-notes/wx-mpu-random-primes-f05-a1.md, src/random/miller_rabin_random.rs, src/random/mod.rs, src/random/vecsample.rs, tests/compat_random.rs, tests/prop_random.rs.
- 2026-10-17T07:13:32Z launcher: no commit for 60 s; 907 s elapsed. Files changed so far: benches/random.rs, docs/worker-notes/wx-mpu-random-primes-f05-a1.md, src/random/miller_rabin_random.rs, src/random/mod.rs, src/random/vecsample.rs, tests/compat_random.rs, tests/prop_random.rs.
- 2026-10-17T07:14:33Z launcher: no commit for 61 s; 969 s elapsed. Files changed so far: benches/random.rs, docs/worker-notes/wx-mpu-random-primes-f05-a1.md, src/random/miller_rabin_random.rs, src/random/mod.rs, src/random/vecsample.rs, tests/compat_random.rs, tests/prop_random.rs.
- 2026-10-17T07:15:33Z launcher: no commit for 60 s; 1029 s elapsed. Files changed so far: benches/random.rs, docs/worker-notes/wx-mpu-random-primes-f05-a1.md, src/random/miller_rabin_random.rs, src/random/mod.rs, src/random/vecsample.rs, tests/compat_random.rs, tests/prop_random.rs.
- 2026-10-17T07:16:33Z launcher: no commit for 60 s; 1089 s elapsed. Files changed so far: benches/random.rs, docs/worker-notes/wx-mpu-random-primes-f05-a1.md, src/random/miller_rabin_random.rs, src/random/mod.rs, src/random/vecsample.rs, tests/compat_random.rs, tests/prop_random.rs.
- 2026-10-17T07:17:35Z launcher: no commit for 61 s; 1150 s elapsed. Files changed so far: benches/random.rs, docs/worker-notes/wx-mpu-random-primes-f05-a1.md, src/random/miller_rabin_random.rs, src/random/mod.rs, src/random/vecsample.rs, tests/compat_random.rs, tests/prop_random.rs.
- 2026-10-17T07:17:35Z launcher: about 60 s to the time limit of 20 minutes; 1150 s elapsed. Files changed so far: benches/random.rs, docs/worker-notes/wx-mpu-random-primes-f05-a1.md, src/random/miller_rabin_random.rs, src/random/mod.rs, src/random/vecsample.rs, tests/compat_random.rs, tests/prop_random.rs.
- 2026-10-17T07:18:36Z launcher: no commit for 61 s; 1211 s elapsed. Files changed so far: benches/random.rs, docs/worker-notes/wx-mpu-random-primes-f05-a1.md, src/random/miller_rabin_random.rs, src/random/mod.rs, src/random/vecsample.rs, tests/compat_random.rs, tests/prop_random.rs.
