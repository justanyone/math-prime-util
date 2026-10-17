I will compare the existing native certificate verifier with MPU 0.75.
I will reuse the merged arithmetic dependencies and keep changes in the proving task paths.
I will add regression tests for parsing, proof validation, and independent repeated calls.
I will run focused offline tests and the required gate. The launcher owns commits.
Compared the existing parser with MPU; found missing target/proof linkage and am checking native proof types before adding regressions.
Added failing regressions for BLS15, signed parameters, case-insensitive fields, and out-of-order factor indices; focused tests are running.
Implemented native BLS15 with modular discriminant arithmetic and strict ordered indices; moved the verifier to its own verify_prime.rs module.
The pre-change regressions failed as expected. Parser regressions now pass; corrected the BLS15 test witness after its Lucas value failed the theorem.
