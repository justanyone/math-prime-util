Continuing the existing estimate mutation regression tests; no existing test files will be edited.
Check remaining mutants against public callers and MPU reference algorithms, distinguishing wrong bounds from equivalent or non-tight changes.
Add reachable regression cases in tests/mutants_estimates.rs, then run focused tests, targeted cargo-mutants, and the required gate.
Do not mark this task READY; record unresolved mutants and environment limitations explicitly.
Reviewed MPU lucky fits and public call domains: the n=7 simple_approx comparison is unreachable; harmonic tiny-term comparison appears f64-equivalent. Preparing targeted mutation baseline.
Added lucky fit boundary sandwich tests through 4e9 and independent fourth-almost-prime checks; focused tests and all 13 requested mutations are next.
Focused regression suite passes all 11 tests. cargo-mutants rejected --offline; restarted with CARGO_NET_OFFLINE=true. Required gate is running independently.
MPU inverse_interpolate.c::interpolate_with_approx confirms that interpolation is only a starting guess followed by exact count correction; comparing branch reachability before claiming equivalent mutants.
Formal branch argument: finite interpolation bounds were sampled only with count error >=20,000, so a bracket spanning both sides has width >=40,000. The span<=19,999 midpoint arm is unreachable; other guess changes require runtime/performance evidence, not a different exact-answer assertion.
