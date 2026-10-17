# MPU-TOTIENTS-F04 notes

Port `ramanujan_sum` and `valuation` from MPU 0.75 totients/moebius into `src/arith/`, each in its own file, plus compat and property tests.

Dependencies are already on this tree (arith module, euler_phi). I will copy the C method, cite the C function, and match the fixture family `arith`.

Next: read PLAN.md sections 5–8, `src/arith/mod.rs`, and the C functions, then implement.

Read PLAN 5–8 and the live functions in `src/arith/mod.rs`. XS returns 0 for a zero argument and wraps the product in an IV. `util.c::valuation` uses `ctz` for base 2 and a power loop. Fixtures already expect `None` for n = 0 and a panic for k < 2.

Next: move each function into its own file, keep the C method, and add property, validation, and concurrency tests.
- 2026-10-17T19:01:45Z launcher: no commit for 61 s; 91 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-totients-f04-a1.md.
- 2026-10-17T19:02:46Z launcher: no commit for 60 s; 152 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-totients-f04-a1.md.
- 2026-10-17T19:03:46Z launcher: no commit for 60 s; 212 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-totients-f04-a1.md.
