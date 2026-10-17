Port the named almost-prime and omega-prime functions from the C reference.
Check existing dependencies and fixture inputs before choosing module boundaries.
Keep tools, fixtures, crate root, and generated status unchanged as instructed.
Run focused offline tests and the repository gate; record any remaining gaps.

Dependencies checked; existing prime counts can support recursive exact counting. Bounds also need MPU coefficient tables and prime/semiprime approximations.

Implemented exact recursive counts, ordered almost-prime construction, omega factor-sieve windows, and rank inversion. Next: coefficient bounds and compatibility tests.

All twelve public functions and fixture adapters are now written; focused offline compatibility tests are running. Bounds reuse existing prime-bound helpers.

Added algebraic rank/count/range tests, repeated and concurrent calls, native boundary tests, and Criterion benchmarks. Compatibility and gate jobs are in progress.

The first gate reached clippy and flagged C coefficient formatting and bounded numeric casts. Preserved C constants with narrow lint allowances and replaced wrapping casts.

About five minutes remain in the session budget. Focused almost-prime tests and property tests are running; the second gate checks the lint fixes.

Property suite passed: 9 tests, including existing family properties. Fixed benchmark semicolon lints; native oracle comparison is still running.

Exact almost-prime counts pass the oracle. Three count estimates fail at u64::MAX because existing f64 prime estimates differ from MPU long-double outputs. Investigating nth compatibility separately.

Compensated large-input prime estimates now preserve low integer bits. This keeps small-input behavior unchanged while testing the long-double dependency gap.
