# MPU-ECM-F01 ecm_factor

Will port Montgomery ECM `ecm_factor` from MPU 0.75 `factor.c` into `src/factor/`, matching stage-1 and stage-2 parameters. Dependencies are now on origin/dev-001; reassess `src/factor/ecm.rs` and fixtures before editing. Compat tests use `tests/fixtures/factor.tsv`. No new Cargo deps. No git writes.
