Port the four nth prime-power functions from MPU 0.75.
Use the existing count and integer helpers and preserve the C interpolation methods.
Add fixture checks, algebraic properties, and hot-function benchmarks.
Leave tools, fixtures, STATUS.md, and the crate root unchanged.
Read the C interpolation and identified missing prime-count estimate helpers; port these privately within the task files.
Implemented all four entry points and private count estimates/interpolation; added oracle, rank, bounds, repeat-call, concurrency, and boundary checks.
Fixed primality return-type mismatch; focused fixture tests and the first gate run are in progress offline.
All 292 target fixture cases and both prime-power property tests pass. Corrected generated table literal style; rerunning the offline gate.
Fixture and property checks are green; added formula-transition and interpolation regressions and matched prime-power membership to MPU maximal perfect-power roots.
Focused unit tests pass across bound formula transitions, interpolation plateaus/jumps, and native limits; the final gate is running.
The gate passed formatting, strict Clippy, and the Rust 1.85 build; full-suite tests are still running offline. No blocked checks so far.
- 2026-10-17T06:52:15Z launcher: no commit for 61 s; 274 s elapsed. Files changed so far: benches/families.rs, docs/worker-notes/wx-mpu-prime-powers-f03.md, src/families/mod.rs, src/families/nth_prime_power.rs, src/families/nth_prime_power_approx.rs, src/families/nth_prime_power_lower.rs, src/families/nth_prime_power_upper.rs, tests/compat_families.rs, tests/prop_families.rs.
The full-suite runs remain in existing prime-count tests; all new focused tests passed, and no code changes are pending.
