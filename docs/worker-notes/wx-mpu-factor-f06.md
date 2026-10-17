Port the four factor predicates from the MPU 0.75 C reference.
Keep each predicate in its own file and use the existing factor fixtures.
Leave generated STATUS.md, fixtures, tooling, and dependencies unchanged.
Run focused offline tests, then the repository gate; record any blockers.
The predicates live in util.c, despite the task naming factor.c; ported their exact filters and base enumeration, with private Brent support because factor is absent.
Added four fixture tests, independent algebraic properties, u64 boundary cases, and criterion benchmarks; focused offline tests are running.
All supported fixture cases pass. The first property incorrectly excluded negative quasi bases; fixed the independent law and added 2093 as a regression for base -1.
Focused fixture and property tests pass. Gate found only benchmark style warnings; corrected them and added direct tests for Brent success, recovery, and exhaustion.
