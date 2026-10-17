Port the five misc gap functions from the local MPU 0.75 reference.
Keep each function in its own file and reuse existing native arithmetic and sieve code.
Add oracle, unit, property, and benchmark coverage without dependencies or fixture changes.
Run focused offline tests, then the required gate; use the repository status tool.

The reference has no Higgs symbol; port the four available functions and document the missing algorithm.

Four functions and their tests are written. Next run focused offline tests and formatting.

Focused compilation found that is_prime returns u8; fixed the property predicate. The status tool ticked only the four implemented functions.

Oracle, four property tests, and six unit tests passed offline. The full gate is running; Higgs remains blocked by its absent reference.

The gate found two pedantic test-style lints. Replaced string collection and empty assertions; rerun the gate next.

Fixed the benchmark macro documentation lint. Final full-suite tests and the gate are running offline.

Status check is current (four new ticks). The checkout has no origin/dev ref, so that diff check is unavailable; the working-tree whitespace check passed.

Gate passed fmt, pedantic Clippy, and MSRV. The existing compatibility suite is still running; no failures have appeared.
- 2026-10-17T09:44:06Z launcher: no commit for 60 s; 332 s elapsed. Files changed so far: STATUS.md, benches/misc_gap.rs, docs/worker-notes/wx-mpu-gap-misc-1.md, src/lib.rs, src/misc_gap/floor_sum.rs, src/misc_gap/mod.rs, src/misc_gap/print_primes.rs, src/misc_gap/remove_factors.rs, src/misc_gap/remove_factors_exp.rs, tests/compat_misc_gap.rs, tests/prop_misc_gap.rs.

The full runs are still in the existing count oracle tests; focused misc-gap tests already passed. No new code changes are needed.
