# MPU-REAL-G01 notes

Port ExponentialIntegral, LogarithmicIntegral, and RiemannZeta from reference/mpu real.c into src/real/, one file each. Match existing module style, compat fixtures, and property tests. Do not change tools, fixtures, or other functions.

Next: read PLAN.md sections 5–8, existing src/real, fixture rows, and the C functions.
Read PLAN 5–8, real.c Ei/Li/ld_riemann_zeta, and the snake_case ports already in src/real/mod.rs. Those ports match the C methods. STATUS ticks only `pub fn ExponentialIntegral` style names, same pattern as Pi, LambertW, and RiemannR.
Next: add one wrapper file each, property laws, run status.py, then compat tests.
Added ExponentialIntegral, LogarithmicIntegral, and RiemannZeta wrappers. Pointed compat and benches at those names. Added purity, validation, and algebraic tests in tests/prop_real.rs.
Next: run tools/test.sh on compat_real and prop_real.
compat_real 10 passed, prop_real 12 passed. STATUS ticks the three names (3 lines). Next: tools/gate.sh.
- 2026-10-17T14:47:53Z launcher: no commit for 64 s; 126 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-real-g01-a1.md.
- 2026-10-17T14:48:56Z launcher: no commit for 61 s; 189 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-real-g01-a1.md.
- 2026-10-17T14:49:58Z launcher: no commit for 60 s; 251 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-real-g01-a1.md.
- 2026-10-17T14:51:38Z launcher: no commit for 60 s; 351 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-real-g01-a1.md.
- 2026-10-17T14:53:47Z launcher: no commit for 64 s; 481 s elapsed. Files changed so far: STATUS.md, benches/real.rs, docs/worker-notes/wx-mpu-real-g01-a1.md, src/real/exponential_integral.rs, src/real/logarithmic_integral.rs, src/real/mod.rs, src/real/riemann_zeta.rs, tests/compat_real.rs, tests/prop_real.rs.
- 2026-10-17T14:55:21Z launcher: no commit for 61 s; 574 s elapsed. Files changed so far: STATUS.md, benches/real.rs, docs/worker-notes/wx-mpu-real-g01-a1.md, src/real/exponential_integral.rs, src/real/logarithmic_integral.rs, src/real/mod.rs, src/real/riemann_zeta.rs, tests/compat_real.rs, tests/prop_real.rs.
- 2026-10-17T14:56:24Z launcher: no commit for 60 s; 638 s elapsed. Files changed so far: STATUS.md, benches/real.rs, docs/worker-notes/wx-mpu-real-g01-a1.md, src/real/exponential_integral.rs, src/real/logarithmic_integral.rs, src/real/mod.rs, src/real/riemann_zeta.rs, tests/compat_real.rs, tests/prop_real.rs.
- 2026-10-17T14:57:41Z launcher: no commit for 64 s; 715 s elapsed. Files changed so far: STATUS.md, benches/real.rs, docs/worker-notes/wx-mpu-real-g01-a1.md, src/real/exponential_integral.rs, src/real/logarithmic_integral.rs, src/real/mod.rs, src/real/riemann_zeta.rs, tests/compat_real.rs, tests/prop_real.rs.
- 2026-10-17T14:58:46Z launcher: no commit for 63 s; 779 s elapsed. Files changed so far: STATUS.md, benches/real.rs, docs/worker-notes/wx-mpu-real-g01-a1.md, src/real/exponential_integral.rs, src/real/logarithmic_integral.rs, src/real/mod.rs, src/real/riemann_zeta.rs, tests/compat_real.rs, tests/prop_real.rs.
