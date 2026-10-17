Port the four assigned Ramanujan functions from MPU 0.75.
Keep each public function in its own file and preserve the C bounds and tables.
Use the existing families fixtures without changing tools or fixture data.
Run focused offline tests and the repository gate; record any blockers here.
Read the C bounds and Noe sieve; private dependencies are absent and will be included within assigned files.
Implemented the lower-bound tables and Noe range sieve. Next: interpolation and fixture tests.
All four functions and compat/property tests are written. Focused offline tests are running; zero, repeat and concurrent calls are covered.
Added threshold tests for the C parameter tables, prime-bound overflow sentinels, and count-bound regions.
First focused build found a u8 primality predicate in the test; corrected it. Clippy numeric-table formatting and checked indices are corrected.
All nth-function oracle cases pass. The max-u64 count case exposed f64 Li error; replacing that private path with compensated extended precision.
The private extended-precision Ei series now matches MPU Li rounding at u64::MAX. Threshold unit tests pass. Fixed numeric formatting before the final gate.
All four compat tests and both property tests pass, including max-u64 count rounding. Added unrepresentable-index checks; no fixture skips.
Focused tests pass after overflow checks. Final gate fixes are limited to benchmark semicolons and criterion macro documentation lint.
Formatting, pedantic Clippy, and MSRV 1.85 build pass. The full offline test and documentation gate is running.
Diff whitespace check passes. Full gate is still running the pre-existing count compatibility tests.
All new tests remain green. Waiting for the existing prime-count compatibility suite in the full gate; no network or external files were changed.
The full gate is spending over a minute in existing chebyshev, nth-prime and prime-count oracle tests. New family tests already passed offline.
The existing Chebyshev oracle passed; the full gate continues with nth-prime and prime-count cases.
The existing nth-prime and prime-count oracle cases are still running. The implementation is unchanged while the full gate validates it.
About five minutes remain in the worker budget. All assigned implementation and tests are complete; the full gate is still in existing count oracle cases.
