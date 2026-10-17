# MPU-COMBINAT

Port combinatorics from MPU 0.75 (util.c, real.c, intcomplexity.c, congruent_numbers.c) into src/combinat/.

Why: origin/dev now has the int/modular patterns and combinat fixtures. Match those APIs, cite C functions, return None on u64 overflow.

Next: read PLAN.md §§5–8, existing module patterns, fixture columns, then implement and test.

Read PLAN §§5–8, compat loader, and MPU `util.c` / `intcomplexity.c` / `congruent_numbers.c`. Fixtures mix u64 values, big integers, `undef`, and `ERROR`.

API: `Option<u128>` or `Option<i128>` while the value fits, `None` past that (and for MPU `undef`). Panics follow MPU croaks. `is_polygonal` of the 42-digit n is skipped (does not fit `u128`).

Wrote `src/combinat/{mod,arith,special}.rs`, compat and property tests, and a bench. Next: compile and fix failures.
- 2026-10-17T00:13:11Z launcher: no commit for 61 s; 91 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:14:11Z launcher: no commit for 60 s; 151 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:15:11Z launcher: no commit for 60 s; 211 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:16:13Z launcher: no commit for 61 s; 273 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:17:13Z launcher: no commit for 60 s; 333 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:18:13Z launcher: no commit for 60 s; 393 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:19:14Z launcher: no commit for 61 s; 454 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:20:14Z launcher: no commit for 60 s; 514 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:21:14Z launcher: no commit for 60 s; 574 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:22:16Z launcher: no commit for 61 s; 635 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:23:16Z launcher: no commit for 60 s; 695 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:24:16Z launcher: no commit for 60 s; 755 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:25:17Z launcher: no commit for 61 s; 817 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:26:18Z launcher: no commit for 60 s; 877 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:27:18Z launcher: no commit for 60 s; 937 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:28:19Z launcher: no commit for 61 s; 998 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:29:19Z launcher: no commit for 60 s; 1058 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:30:19Z launcher: no commit for 60 s; 1118 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:31:20Z launcher: no commit for 61 s; 1180 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:32:20Z launcher: no commit for 60 s; 1240 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:33:20Z launcher: no commit for 60 s; 1300 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:34:22Z launcher: no commit for 61 s; 1361 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:35:22Z launcher: no commit for 60 s; 1421 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md.
- 2026-10-17T00:38:53Z launcher: no commit for 60 s; 1633 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-combinat-a1.md, src/combinat/arith.rs, src/combinat/mod.rs, src/combinat/special.rs.
- 2026-10-17T00:41:25Z launcher: no commit for 60 s; 1784 s elapsed. Files changed so far: benches/combinat.rs, docs/worker-notes/wx-mpu-combinat-a1.md, src/combinat/arith.rs, src/combinat/mod.rs, src/combinat/special.rs, tests/compat_combinat.rs, tests/prop_combinat.rs.
