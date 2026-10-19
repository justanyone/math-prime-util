Implement all twelve bigint digit functions with native API parity.
Read PLAN sections 5–8 and use the vendored MPU algorithms.
Add pure-Perl oracle fixtures and property tests behind bigint.
Run offline focused tests and the required gate; record blockers here.
Implemented all twelve functions; retained native validation and Option conventions. Next add oracle and parity tests.
Added fixture generator and native parity/round-trip/validation tests. Local Perl bootstrap is absent; using vendored PP offline instead.
Vendored MPU PP generated bigint_digits.tsv offline. Focused tests are building; no dependencies or native APIs changed.
Focused build found an unavailable direct num_traits import; replaced it with standard TryFrom. About five minutes remain; finishing validation and documenting host/coverage limits.
First replay exposed empty-list fixture encoding, not arithmetic; generator now serializes digit lists explicitly. Fixed the clippy clone assignment warning.
All focused tests passed: full oracle replay, u64 parity, wide round trips, and invalid input checks. Gate is running; coverage tool is unavailable.
Reviewed state/concurrency: functions have no shared mutable state or I/O. Generator writes only its requested family; fixture replay is deterministic.
Fmt and default/bigint/GMP clippy checks passed. Whole-suite validation is compiling integration tests offline.
Added u64::MAX-base round trips and extreme exponent checks for conversion boundaries. Full suite and remaining gate steps continue offline.
All five focused tests now pass, including large bases and exponent boundaries. All four clippy feature sets passed; the gate is building MSRV dependencies.
Gate MSRV build passed and full tests are executing. No upstream discrepancies appeared in the bigint digit oracle.
Full test execution continues; no failure has been reported. Coverage >=95% cannot be measured without cargo-llvm-cov, and aragog/wonko are inaccessible under the no-network rule.
Waiting for existing count-family integration tests in the full gate; bigint digit focused tests remain green.
Full suite is spending over a minute in the existing prime_count oracle. No bigint digit failure; waiting for gate completion.
