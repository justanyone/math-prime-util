# MPU-RANDOM-PRIMES-F05

Port `vecsample` and `miller_rabin_random` into `src/random/` as their own files. Native `u64` only; bit sizes above 64 return `None`. Follow MPU 0.75 algorithms from `reference/mpu` and the existing module pattern. Add compat and property tests. Do not edit STATUS.md, lib.rs, tools, or fixtures.

- Start: notes written. Next: read PLAN §§5–8, existing `src/random`, and the two functions in the C/Perl sources.
- 2026-10-17T06:59:55Z launcher: no commit for 61 s; 91 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-random-primes-f05-a1.md.
