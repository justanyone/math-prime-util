I will compare the existing native certificate verifier with MPU 0.75.
I will reuse the merged arithmetic dependencies and keep changes in the proving task paths.
I will add regression tests for parsing, proof validation, and independent repeated calls.
I will run focused offline tests and the required gate. The launcher owns commits.
Compared the existing parser with MPU; found missing target/proof linkage and am checking native proof types before adding regressions.
Added failing regressions for BLS15, signed parameters, case-insensitive fields, and out-of-order factor indices; focused tests are running.
Implemented native BLS15 with modular discriminant arithmetic and strict ordered indices; moved the verifier to its own verify_prime.rs module.
The pre-change regressions failed as expected. Parser regressions now pass; corrected the BLS15 test witness after its Lucas value failed the theorem.
Focused compat and property tests passed. Added MPU native fallback and whitespace properties; approximately five minutes remain for lint, final tests, and review.
Unit and focused tests pass. Added concurrent signed-proof calls interleaved with malformed input and duplicate proof blocks to check independence.
The gate passed fmt, clippy, and the MSRV build and is running the full tests. Added boundary cases for native BLS15 and range validation.
Final focused run passed all 15 compatibility and 15 property tests. Diff whitespace checks pass; the full gate is still running offline.
No focused failures remain. The full gate is waiting in the existing iterator compatibility tests; no new verifier code is running there.
- 2026-10-17T19:52:56Z launcher: no commit for 60 s; 336 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-aks-proving-f02-a2.md, src/proving/mod.rs, src/proving/verify_prime.rs, tests/compat_primality.rs, tests/prop_primality.rs.
