# MPU-ECM-F01 ecm_factor

Will port Montgomery ECM `ecm_factor` from MPU 0.75 `factor.c` into `src/factor/`, matching stage-1 and stage-2 parameters. Dependencies are now on origin/dev-001; reassess `src/factor/ecm.rs` and fixtures before editing. Compat tests use `tests/fixtures/factor.tsv`. No new Cargo deps. No git writes.

Read PLAN 5-8, `src/factor/ecm.rs`, XS `ecm_factor` alias, and `tinyecm64`. Existing port looks faithful (Suyama sigmas, stage 1/2, B2=0 means 20*B1). Next: compare sigma tables, then `tools/test.sh ecm`.
Sigma table matches C (400 values). Stage 2 now sieves primes once per call and returns 0 when the window count is 0, as in `tinyecm64_stage2`. Next: `tools/test.sh ecm`.
`tools/test.sh ecm` exited 0. `cargo fmt` and `cargo clippy -D warnings` are clean. Next: `tools/gate.sh`.
- 2026-10-17T19:01:49Z launcher: no commit for 61 s; 91 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-ecm-f01-a1.md.
- 2026-10-17T19:02:49Z launcher: no commit for 60 s; 151 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-ecm-f01-a1.md.
- 2026-10-17T19:03:49Z launcher: no commit for 60 s; 211 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-ecm-f01-a1.md.
- 2026-10-17T19:05:54Z launcher: no commit for 60 s; 336 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-ecm-f01-a1.md.
- 2026-10-17T19:06:57Z launcher: no commit for 61 s; 399 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-ecm-f01-a1.md.
- 2026-10-17T19:09:31Z launcher: no commit for 60 s; 553 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-ecm-f01-a1.md, src/factor/ecm.rs.
