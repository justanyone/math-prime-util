# MPU-DIVISOR-SUMS-F04

Port `qnr`, `is_qr`, and `is_fundamental` from MPU 0.75 `util.c` into `src/arith/`, each in its own file, with compat and property tests.

Dependencies are on origin/dev. Read current arith code and the C algorithms, then add only these three functions.

Next: locate PLAN.md sections 5–8, existing arith module shape, and the C functions.
Read PLAN 5–8. `qnr`, `is_qr`, and `is_fundamental` already live in `src/arith/mod.rs` with docs, and compat tests exist. `qnr` oracle test passed. Next: `is_qr`, `is_fundamental`, property tests, then the gate.
Moved the three functions into `src/arith/{qnr,is_qr,is_fundamental}.rs`, dropped the unused `next_prime` import, and added residue laws plus repeat/race tests and benches. Next: fmt and focused tests, then the gate.
Compat and property tests passed. A prime-sampling proptest rejected too often; it is now a fixed prime table. Starting the gate. About five minutes of budget may remain.
- 2026-10-17T18:59:35Z launcher: no commit for 61 s; 121 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-divisor-sums-f04-a1.md.
- 2026-10-17T19:00:35Z launcher: no commit for 60 s; 182 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-divisor-sums-f04-a1.md.
- 2026-10-17T19:01:36Z launcher: no commit for 60 s; 242 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-divisor-sums-f04-a1.md.
- 2026-10-17T19:03:07Z launcher: no commit for 60 s; 334 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-divisor-sums-f04-a1.md.
- 2026-10-17T19:04:08Z launcher: no commit for 60 s; 394 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-divisor-sums-f04-a1.md.
- 2026-10-17T19:09:21Z launcher: no commit for 60 s; 707 s elapsed. Files changed so far: benches/arith.rs, docs/worker-notes/wx-mpu-divisor-sums-f04-a1.md, src/arith/is_fundamental.rs, src/arith/is_qr.rs, src/arith/mod.rs, src/arith/qnr.rs, tests/prop_arith.rs.
