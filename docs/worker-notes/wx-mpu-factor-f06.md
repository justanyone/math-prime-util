Port the four factor predicates from the MPU 0.75 C reference.
Keep each predicate in its own file and use the existing factor fixtures.
Leave generated STATUS.md, fixtures, tooling, and dependencies unchanged.
Run focused offline tests, then the repository gate; record any blockers.
The predicates live in util.c, despite the task naming factor.c; ported their exact filters and base enumeration, with private Brent support because factor is absent.
