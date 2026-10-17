# MPU-REAL-G01 notes

Port ExponentialIntegral, LogarithmicIntegral, and RiemannZeta from reference/mpu real.c into src/real/, one file each. Match existing module style, compat fixtures, and property tests. Do not change tools, fixtures, or other functions.

Next: read PLAN.md sections 5–8, existing src/real, fixture rows, and the C functions.
Read PLAN 5–8, real.c Ei/Li/ld_riemann_zeta, and the snake_case ports already in src/real/mod.rs. Those ports match the C methods. STATUS ticks only `pub fn ExponentialIntegral` style names, same pattern as Pi, LambertW, and RiemannR.
Next: add one wrapper file each, property laws, run status.py, then compat tests.
- 2026-10-17T14:47:53Z launcher: no commit for 64 s; 126 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-real-g01-a1.md.
- 2026-10-17T14:48:56Z launcher: no commit for 61 s; 189 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-real-g01-a1.md.
- 2026-10-17T14:49:58Z launcher: no commit for 60 s; 251 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-real-g01-a1.md.
- 2026-10-17T14:51:38Z launcher: no commit for 60 s; 351 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-real-g01-a1.md.
