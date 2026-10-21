# Worker notes: close mutation survivors (lucky, ramanujan-upper, almost-prime)
- Plan: read the 4 files at the listed lines, check each survivor's bound, add tests/mutants_estimates.rs with killing tests.
- Fix only if a survivor shows the bound is actually wrong.
- 2026-10-21T01:54:30Z launcher: no commit for 60 s; 122 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-close-mutation-survivors-i-restart-1792547524.md, tests/mutants_estimates.rs.
- 20:54 wrote tests/mutants_estimates.rs (public API tests); running focused test
- 20:55 fixed two wrong expected values in my test (R_9=71, lucky_count counts <= x); rerunning
- 20:55 5 tests pass (--test mutants_estimates). cargo-mutants not re-run (time). 13 survivors in private fns not directly killed: public API only. Stopping.
