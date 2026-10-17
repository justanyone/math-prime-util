# pc1-mpu-prime-counts-g02-t0958

Continuing a PARTIAL run. `nth_prime_lower`, `nth_prime_upper`, and `nth_prime_approx` fail compat fixtures. Plan: read PLAN.md §§5–8 and the three count modules, match the C reference bounds, fix the mismatches, then run focused tests, `prop_count`, benches if required, and `tools/gate.sh`. Tick STATUS only for functions that pass. Do not mark READY.

- t0: notes written; next is PLAN sections 5–8 and the three nth_prime sources.
- t1: compat fails because n<2e6/15360 returns the exact prime; C binary-searches prime_count_upper/lower. Large upper bounds need more than f64 (1e14 off by 1+). Approx off by 1 at 1e13. Next: port the search and a double-double Axler evaluation.
- 2026-10-17T15:00:47Z launcher: no commit for 61 s; 91 s elapsed. Files changed so far: docs/worker-notes/pc1-mpu-prime-counts-g02-t0958.md.
- 2026-10-17T15:01:47Z launcher: no commit for 60 s; 151 s elapsed. Files changed so far: docs/worker-notes/pc1-mpu-prime-counts-g02-t0958.md.
- 2026-10-17T15:02:48Z launcher: no commit for 60 s; 211 s elapsed. Files changed so far: docs/worker-notes/pc1-mpu-prime-counts-g02-t0958.md.
- 2026-10-17T15:03:49Z launcher: no commit for 61 s; 273 s elapsed. Files changed so far: docs/worker-notes/pc1-mpu-prime-counts-g02-t0958.md.
- 2026-10-17T15:04:52Z launcher: no commit for 60 s; 335 s elapsed. Files changed so far: docs/worker-notes/pc1-mpu-prime-counts-g02-t0958.md.
- 2026-10-17T15:05:53Z launcher: no commit for 60 s; 397 s elapsed. Files changed so far: docs/worker-notes/pc1-mpu-prime-counts-g02-t0958.md.
- 2026-10-17T15:06:57Z launcher: no commit for 62 s; 461 s elapsed. Files changed so far: docs/worker-notes/pc1-mpu-prime-counts-g02-t0958.md.
- 2026-10-17T15:07:58Z launcher: no commit for 60 s; 522 s elapsed. Files changed so far: docs/worker-notes/pc1-mpu-prime-counts-g02-t0958.md.
- 2026-10-17T15:09:00Z launcher: no commit for 60 s; 583 s elapsed. Files changed so far: docs/worker-notes/pc1-mpu-prime-counts-g02-t0958.md.
- 2026-10-17T15:10:08Z launcher: no commit for 66 s; 652 s elapsed. Files changed so far: docs/worker-notes/pc1-mpu-prime-counts-g02-t0958.md.
- 2026-10-17T15:11:09Z launcher: no commit for 60 s; 712 s elapsed. Files changed so far: docs/worker-notes/pc1-mpu-prime-counts-g02-t0958.md.
- 2026-10-17T15:12:11Z launcher: no commit for 60 s; 774 s elapsed. Files changed so far: docs/worker-notes/pc1-mpu-prime-counts-g02-t0958.md.
- 2026-10-17T15:12:48Z launcher: about 60 s to the time limit of 14 minutes; 811 s elapsed. Files changed so far: docs/worker-notes/pc1-mpu-prime-counts-g02-t0958.md.
