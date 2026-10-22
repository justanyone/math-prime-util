Continuation: review existing mutation regression tests and prior validation evidence.
Run the focused estimates tests, then complete tools/gate.sh and record its result.
Confirm all 13 target mutants have recorded outcomes; rerun the focused mutation sweep as needed.
Keep existing tests and production algorithms unchanged unless verification reveals a bug.
Focused filter correction: private helper tests use estimate_mutation_tests; checking prior successful mutation logs and awaiting the fresh gate.
Verified archived outcomes.json: exact 13 targets caught, zero missed/timeouts/unviable; private helper mutations match the current extracted helper names. Fresh focused tests and gate are running.
All 13 archived outcomes map to the requested sites. Preparing a fresh four-file sweep while the required gate waits for the first test build.
