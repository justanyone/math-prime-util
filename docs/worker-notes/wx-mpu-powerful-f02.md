Reassess the four existing functions against MPU 0.75 and their dependencies.
Move each owned public function to its own module file, preserving sibling functions.
Use the existing families oracle and add regression tests for any defects found.
Run focused offline tests and the gate; record unavailable checks here.
The existing oracle and property suites cover all four functions. Split their definitions into owned files; retain shared sieve helpers for sibling sum functions.
Restored the C predicate shortcuts and factorint dependency. Replaced the truncated zeta sum with the crate's port of ld_riemann_zeta. Focused offline tests are running.
Ported the C linear interpolation, Ridder refinement, and final binary search for nth_powerful. Added input, overflow, count-step, repeat-call, and concurrent-call tests; no shared state is added.
The gate passed formatting and is waiting for the focused test build lock. Existing fixture files and their generator stay unchanged because this family is already present.
No semantic oracle mismatch has appeared yet; test output is buffered by tools/test.sh. The focused build still holds the build lock, so the gate cannot advance.
Formatting passed. Focused compatibility/property tests, interpolation unit tests, and the required offline gate are queued on the same Cargo build directory.
