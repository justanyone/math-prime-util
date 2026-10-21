Review the inherited estimate regressions and the remaining equivalent harmreal mutant.
Check boundary behavior against the read-only MPU reference before changing algorithms.
Run focused tests, the requested four-file mutation check, and the repository gate.
Record equivalent mutants explicitly; do not claim READY or force an observable API change for a test.

Reviewed inherited regressions and MPU PP.pm::harmreal: cutoff is a performance contract, numerically equivalent under f64; focused tests started.

Extracted the unchanged PP.pm harmonic correction loop into an iterator helper; the new test checks early termination without changing numerical results. Corrected its expected first-cutoff term analytically (term 6 at n=2003).

All 37 focused estimate tests pass. Starting the 13 requested mutation cases (harmreal cutoff now in harmonic_correction); repository gate runs separately. The listed Ramanujan comparison is at 15360, not the equivalent 306 branch.

Mutation runner found exactly 13 requested cases. Clippy default and bigint passed; gmp/all-feature checks and mutation builds continue. Boundary triage: harmonic cutoff affects work only; lucky simple_approx is an estimate, and the listed divisions are not exact-result paths.
