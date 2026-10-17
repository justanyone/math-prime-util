Port the four factor predicates from the MPU 0.75 C reference.
Keep each predicate in its own file and use the existing factor fixtures.
Leave generated STATUS.md, fixtures, tooling, and dependencies unchanged.
Run focused offline tests, then the repository gate; record any blockers.
The predicates live in util.c, despite the task naming factor.c; ported their exact filters and base enumeration, with private Brent support because factor is absent.
Added four fixture tests, independent algebraic properties, u64 boundary cases, and criterion benchmarks; focused offline tests are running.
All supported fixture cases pass. The first property incorrectly excluded negative quasi bases; fixed the independent law and added 2093 as a regression for base -1.
Focused fixture and property tests pass. Gate found only benchmark style warnings; corrected them and added direct tests for Brent success, recovery, and exhaustion.
The direct factor tests exercise large semiprimes, repeated prime powers, prime rejection, and u64::MAX; the gate is rerunning after the benchmark lint fixes.
Corrected the new assert_is_empty lint. Added repeated and concurrent predicate calls to check that no shared state affects results; gate is running again.
