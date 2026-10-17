Reassess the four existing functions against MPU 0.75 and their dependencies.
Move each owned public function to its own module file, preserving sibling functions.
Use the existing families oracle and add regression tests for any defects found.
Run focused offline tests and the gate; record unavailable checks here.
The existing oracle and property suites cover all four functions. Split their definitions into owned files; retain shared sieve helpers for sibling sum functions.
Restored the C predicate shortcuts and factorint dependency. Replaced the truncated zeta sum with the crate's port of ld_riemann_zeta. Focused offline tests are running.
Ported the C linear interpolation, Ridder refinement, and final binary search for nth_powerful. Added input, overflow, count-step, repeat-call, and concurrent-call tests; no shared state is added.
