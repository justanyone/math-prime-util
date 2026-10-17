# MPU-INVERSE-SIGMA-F01

Port `inverse_sigma0` and `inverse_sigma0_count` from MPU 0.75 C (`inverse_sigma0.c`, `inverse_interpolate.c`) into `src/arith/`, one function per file, with compat and property tests.

Dependencies are now on origin/dev-001. Read the current arith module and fixtures first, then write the smallest complete port that matches the C method and the existing test patterns.

Next: read PLAN.md sections 5–8, the named tools, and the existing `src/arith` inverse-sigma code.

Read PLAN 5–8, test.sh, gate.sh, lib.rs. `src/arith/inverse_sigma0.rs` already has both functions (sieve + signature), compat tests, prop test, and a bench. `inverse_interpolate.c` is not called by `inverse_sigma0.c`. Next: run `tools/test.sh tests/compat_arith.rs inverse_sigma0` and fix failures.

Compat oracle and lib unit tests passed (11 lib tests). Clippy `-D warnings` failed on `float_cmp` in `src/count/mod.rs` (chebyshev 0.0) and `src/proving/is_aks_prime.rs` (`-f64::MAX`). Changed those four asserts to `to_bits` so the gate can compile. About five minutes left: run `tools/gate.sh` next.
- 2026-10-17T19:05:22Z launcher: no commit for 61 s; 92 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-inverse-sigma-f01-a1.md.
- 2026-10-17T19:06:22Z launcher: no commit for 60 s; 152 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-inverse-sigma-f01-a1.md.
- 2026-10-17T19:07:22Z launcher: no commit for 60 s; 212 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-inverse-sigma-f01-a1.md.
- 2026-10-17T19:08:54Z launcher: no commit for 60 s; 304 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-inverse-sigma-f01-a1.md.
- 2026-10-17T19:09:55Z launcher: no commit for 60 s; 365 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-inverse-sigma-f01-a1.md.
- 2026-10-17T19:10:57Z launcher: no commit for 61 s; 427 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-inverse-sigma-f01-a1.md.
- 2026-10-17T19:11:57Z launcher: no commit for 60 s; 487 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-inverse-sigma-f01-a1.md.
- 2026-10-17T19:12:57Z launcher: no commit for 60 s; 547 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-inverse-sigma-f01-a1.md.
- 2026-10-17T19:13:59Z launcher: no commit for 61 s; 609 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-inverse-sigma-f01-a1.md.
- 2026-10-17T19:15:59Z launcher: no commit for 60 s; 730 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-inverse-sigma-f01-a1.md, src/count/mod.rs, src/proving/is_aks_prime.rs.
- 2026-10-17T19:17:03Z launcher: no commit for 63 s; 793 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-inverse-sigma-f01-a1.md, src/count/mod.rs, src/proving/is_aks_prime.rs.
- 2026-10-17T19:18:03Z launcher: no commit for 60 s; 853 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-inverse-sigma-f01-a1.md, src/count/mod.rs, src/proving/is_aks_prime.rs.
- 2026-10-17T19:19:33Z launcher: no commit for 60 s; 943 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-inverse-sigma-f01-a1.md, src/count/mod.rs, src/proving/is_aks_prime.rs.
