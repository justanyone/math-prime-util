Continuation: review existing mutation regression tests and prior validation evidence.
Run the focused estimates tests, then complete tools/gate.sh and record its result.
Confirm all 13 target mutants have recorded outcomes; rerun the focused mutation sweep as needed.
Keep existing tests and production algorithms unchanged unless verification reveals a bug.
Focused filter correction: private helper tests use estimate_mutation_tests; checking prior successful mutation logs and awaiting the fresh gate.
Verified archived outcomes.json: exact 13 targets caught, zero missed/timeouts/unviable; private helper mutations match the current extracted helper names. Fresh focused tests and gate are running.
All 13 archived outcomes map to the requested sites. Preparing a fresh four-file sweep while the required gate waits for the first test build.
- 2026-10-22T04:05:24Z launcher: no commit for 60 s; 182 s elapsed. Files changed so far: docs/worker-notes/pc1-mpu-close-mutation-survivors-in-lucky-ramanujan-upper-t2302.md.
Fresh cargo-mutants selected exactly 13 targets on the four source files. Initial zero-match filter finished; corrected private filter is queued, and gate has progressed into clippy checks.
Corrected focused tests passed: 37/37. Gate passed fmt, four clippy feature sets and MSRV, and is running the full suite. Fresh 13-site mutation baseline passed.
Fresh cargo-mutants completed successfully: 13 tested, 13 caught, zero misses/timeouts/unviable. Command used --no-config --all-features -C --offline -C --lib, four explicit -f paths, exact site regex, and -- estimate_mutation_tests.
Full gate is still running count compatibility tests; no failures reported. No source or existing test edits are needed in this continuation.
Gate checkpoint: prime_count_oracle remains active (over 60 seconds); other count oracle tests passed. Fresh mutation result remains 13/13 and focused tests 37/37.
Gate passed the long count suite and advanced through factor oracle tests; no failures. Awaiting remaining compatibility/property tests, doctests and documentation.
Gate reached property tests after compatibility suites passed. No failures reported; awaiting gate completion and the explicit green marker.
