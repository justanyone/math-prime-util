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

Omega count compatibility passes. Large-input prime estimates still differ by integer units; reference real.c is needed to match its truncation, not only mathematical accuracy.

Large-input count lower/upper now pass after matching the C Li return-type rounding. RiemannR matches too; correcting the final mixed float/integer expression for k >= 4.

Found large prefix fixtures that require the C omega recursive-construction branch. Added that branch; narrow arbitrary ranges keep the bounded factor sieve.

Final compatibility and gate runs are active. Clippy, MSRV, and whole-suite execution are being checked offline; earlier stale long prefix runs were stopped after the recursive branch fix.

Added native omega rank overflow checks from the C maximum-count table. Reused the existing prime nth-bound helper for k=1 instead of substituting exact ranks.

Large omega prefixes need a minimum distinct-prime-product check to prune impossible recursive branches. Added the check and reran the prefix oracle test.

Added bounded repeated-factor sieving for narrow almost-prime ranges, matching the reference range algorithm and avoiding prefix construction below lo.

All 26 family compatibility tests pass in 1.73 seconds. All nine family property tests pass. Final gate is running against the current source; generated status, fixtures, tools, and crate root remain unchanged.

About five minutes remain. The complete family oracle suite passes; compensated RiemannR now also follows the reference large-input Moebius/Li branch. The full gate spends minutes in existing count compatibility tests.
