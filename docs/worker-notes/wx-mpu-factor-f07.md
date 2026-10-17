Port the four factor predicates and smooth count from MPU 0.75.
Keep each public function in its own file and reuse available primitives.
Use the committed factor fixtures and algebraic properties for offline checks.
Leave tools, fixtures, STATUS.md, and git metadata unchanged.
The named predicates are in util.c, not factor.c; factor support is absent in this checkout. Use the actual C locations in citations and private factoring support where needed.
Added all four API files and oracle tests. Smooth count retains the C cache tables and recurrence. Next run focused tests and review the missing sibling-factor integration.
Fixed the helper's integer type. Added property tests, subset-sum practical checks, concurrency checks, and benchmark source. Restoring the C roughness p-1 fast path with bounds 500/500.
All fixture and property tests pass. The first gate stopped on clippy's panic-doc lint for bounded unwraps; removing those unwraps before rerunning the gate.
Private Brent retry and limit tests pass, including a near-u64 semiprime. The gate now reaches benchmark/test lints; fixing semicolons, generated benchmark docs, and numeric separators.
