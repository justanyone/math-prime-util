Review the inherited estimate regressions and the remaining equivalent harmreal mutant.
Check boundary behavior against the read-only MPU reference before changing algorithms.
Run focused tests, the requested four-file mutation check, and the repository gate.
Record equivalent mutants explicitly; do not claim READY or force an observable API change for a test.

Reviewed inherited regressions and MPU PP.pm::harmreal: cutoff is a performance contract, numerically equivalent under f64; focused tests started.

Extracted the unchanged PP.pm harmonic correction loop into an iterator helper; the new test checks early termination without changing numerical results. Corrected its expected first-cutoff term analytically (term 6 at n=2003).

All 37 focused estimate tests pass. Starting the 13 requested mutation cases (harmreal cutoff now in harmonic_correction); repository gate runs separately. The listed Ramanujan comparison is at 15360, not the equivalent 306 branch.

Mutation runner found exactly 13 requested cases. Clippy default and bigint passed; gmp/all-feature checks and mutation builds continue. Boundary triage: harmonic cutoff affects work only; lucky simple_approx is an estimate, and the listed divisions are not exact-result paths.

Boundary arithmetic: changing either lucky division to multiplication yields a negative factor (saturating to zero), so its lower bound stays valid but useless. Reversed branch comparisons give smaller lower estimates at 1000001 and 10000001 (70861 vs 70936; 600795 vs 602729). No original wrong bound identified. Full tools/test.sh started as required.

All four clippy feature sets passed; gate is building MSRV. Mutation baseline builds all test targets before applying its focused runtime filter, so compilation is longer than the focused test run. No live services or external files changed.

Triage of the other cases: lucky n=7 changes only the estimate (3 to 2); both public count bounds use independent small-input formulas. Axler threshold/sign tests enforce MPU coefficients; the huge exact R_n boundary is not computed, so no unsupported claim of a violated bound is made. Almost-prime synthetic plateau traces verify bracket guards and retreat arithmetic; no unmutated bug found.

BLOCKER: mutation baseline and full-suite compilation failed with disk quota exceeded. Focused 37-test run and all four clippy feature sets passed. Checking task-created build artifacts, then narrowing mutation build to --lib and disabling debug info if space permits.

Disk use is dominated by task-local debug artifacts (deps plus incremental); no files outside the worktree will be removed. Validation will be retried with debug symbols and incremental compilation disabled.

Stopped the full-suite attempt after quota exhaustion; removed only task-local incremental cache, then cleaned task-local target for a compact gate retry. Retrying exactly 13 mutants with --lib at build time, no debug symbols, and no incremental cache.

Compact validation is running: cleared only worktree target cache; no source behavior or dependencies changed. Mutation selection remains exactly 13; gate restart includes the required full suite.

Compact mutant retry exposed a duplicated --lib argument (build and test options both supplied it); remove the duplicate and rerun with the build option alone.

Corrected the duplicate --lib invocation; fresh focused mutation baseline is compiling. Gate compact retry is progressing independently.
- 2026-10-21T03:55:50Z launcher: no commit for 60 s; 490 s elapsed. Files changed so far: docs/worker-notes/pc1-mpu-close-mutation-survivors-in-lucky-ramanujan-upper-t2247.md, src/real/mod.rs, tests/mutants_estimates.rs.
