Port the four nth prime-power functions from MPU 0.75.
Use the existing count and integer helpers and preserve the C interpolation methods.
Add fixture checks, algebraic properties, and hot-function benchmarks.
Leave tools, fixtures, STATUS.md, and the crate root unchanged.
Read the C interpolation and identified missing prime-count estimate helpers; port these privately within the task files.
Implemented all four entry points and private count estimates/interpolation; added oracle, rank, bounds, repeat-call, concurrency, and boundary checks.
Fixed primality return-type mismatch; focused fixture tests and the first gate run are in progress offline.
All 292 target fixture cases and both prime-power property tests pass. Corrected generated table literal style; rerunning the offline gate.
Fixture and property checks are green; added formula-transition and interpolation regressions and matched prime-power membership to MPU maximal perfect-power roots.
