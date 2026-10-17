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
