Port MPU 0.75 ECM and its Montgomery curve helpers to src/factor.
Read PLAN sections 5–8 and the prescribed test and gate commands.
Keep the C bounds and stages, use existing arithmetic, and leave fixtures and tooling unchanged.
Add compatibility, algebraic, and helper tests, then run the offline gate.
Inspection: reference uses tinyecm64_factor with fixed sigmas; checking the public wrapper defaults before porting.
Confirmed XS defaults: B1=4000, B2=20*B1, 100 curves; wrapper removes factors 2, 3, 5 and returns an unsplit cofactor on failure.
Implemented curve arithmetic, batch normalization, both stages, and the public wrapper; reused the existing portable Montgomery context.
Added oracle, product-law, repeated/concurrent-call, validation, and curve-helper tests plus the ECM benchmark. Focused offline tests started.
Corrected primality status comparisons. Focused tests and the offline gate are running; no fixtures or tooling were changed.
All 57 oracle cases ran. One has a different valid composite-factor partition: reference port gives 49*1856137793, fixture gives 7*12992964551. Test records both partitions explicitly; strict string parity remains a known difference.
Gate passes ECM lint checks but stops on existing clippy::float_cmp errors at src/count/mod.rs:240–242, outside task scope. Running helper tests and the full offline suite next.
Added a zero-curve early return so even maximum bounds do not allocate prime tables; added its regression test. Strict oracle partition equality and the unrelated gate warnings remain open.
Helper tests pass (3/3); compatibility test and four property/state tests pass. Full offline suite is running. No network, dependency, fixture, tooling, or STATUS changes were made.
Final gate attempt still stops in pre-existing count-module float comparisons; no changes made outside the ECM task paths. Waiting for the single full-suite run.
Benchmark target compile check requested while the full offline suite continues; normal gate is blocked before MSRV/docs by count-module lint errors.
Benchmark target compiles successfully offline. Remaining acceptance limits: strict partition equality for the named fixture and a green unmodified repository gate.
Focused checks are complete; the full-suite command has not returned yet. No further source changes are planned. Oracle partition and baseline lint blockers are documented.
Final diff whitespace check passed. All task implementation and focused tests are on disk; full-suite result is pending and the normal gate remains blocked outside task scope.
Full-suite run is still pending; targeted tests, benchmark compile, formatting, and diff whitespace checks pass. Gate MSRV/docs stages cannot run because pre-existing clippy errors stop the script.
Near the final five-minute window: implementation is finished; full-suite output is pending. Preserve the faithful C split and report the named oracle partition difference and out-of-scope gate errors.
- 2026-10-17T08:05:17Z launcher: no commit for 60 s; 575 s elapsed. Files changed so far: benches/factor.rs, docs/worker-notes/wx-mpu-ecm.md, src/factor/ecm.rs, src/factor/mod.rs, tests/compat_factor.rs, tests/prop_factor.rs.
Stopping with ECM implementation, documentation, benchmark, and eight focused tests complete and passing. Full offline suite has not returned. Acceptance remains partial: exact partition parity for 1819015037140 and the normal gate blocked at src/count/mod.rs:240–242. Launcher handles commits.
