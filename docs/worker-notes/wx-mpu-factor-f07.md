Port the four factor predicates and smooth count from MPU 0.75.
Keep each public function in its own file and reuse available primitives.
Use the committed factor fixtures and algebraic properties for offline checks.
Leave tools, fixtures, STATUS.md, and git metadata unchanged.
The named predicates are in util.c, not factor.c; factor support is absent in this checkout. Use the actual C locations in citations and private factoring support where needed.
Added all four API files and oracle tests. Smooth count retains the C cache tables and recurrence. Next run focused tests and review the missing sibling-factor integration.
Fixed the helper's integer type. Added property tests, subset-sum practical checks, concurrency checks, and benchmark source. Restoring the C roughness p-1 fast path with bounds 500/500.
All fixture and property tests pass. The first gate stopped on clippy's panic-doc lint for bounded unwraps; removing those unwraps before rerunning the gate.
Private Brent retry and limit tests pass, including a near-u64 semiprime. The gate now reaches benchmark/test lints; fixing semicolons, generated benchmark docs, and numeric separators.
The targeted oracle, property, and private-support tests pass. Clippy-only fixes are complete; the full offline gate is running. No fixture regeneration or network access is needed.
The gate printed == gate: green: fmt, clippy, MSRV 1.85, all tests, doctests, and docs passed offline. Coverage tools are not installed on PATH, so the coverage threshold remains for CI.
Final check: the required unfiltered tools/test.sh passed offline. All four functions, fixture tests, property/regression tests, and benchmark source are complete; no files in tools, fixtures, or STATUS.md were changed.
