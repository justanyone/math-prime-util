Continuing the existing estimate mutation regression tests; no existing test files will be edited.
Check remaining mutants against public callers and MPU reference algorithms, distinguishing wrong bounds from equivalent or non-tight changes.
Add reachable regression cases in tests/mutants_estimates.rs, then run focused tests, targeted cargo-mutants, and the required gate.
Do not mark this task READY; record unresolved mutants and environment limitations explicitly.
Reviewed MPU lucky fits and public call domains: the n=7 simple_approx comparison is unreachable; harmonic tiny-term comparison appears f64-equivalent. Preparing targeted mutation baseline.
Added lucky fit boundary sandwich tests through 4e9 and independent fourth-almost-prime checks; focused tests and all 13 requested mutations are next.
