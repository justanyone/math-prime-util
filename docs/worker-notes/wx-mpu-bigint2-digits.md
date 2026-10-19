Implement all twelve bigint digit functions with native API parity.
Read PLAN sections 5–8 and use the vendored MPU algorithms.
Add pure-Perl oracle fixtures and property tests behind bigint.
Run offline focused tests and the required gate; record blockers here.
Implemented all twelve functions; retained native validation and Option conventions. Next add oracle and parity tests.
Added fixture generator and native parity/round-trip/validation tests. Local Perl bootstrap is absent; using vendored PP offline instead.
Vendored MPU PP generated bigint_digits.tsv offline. Focused tests are building; no dependencies or native APIs changed.
