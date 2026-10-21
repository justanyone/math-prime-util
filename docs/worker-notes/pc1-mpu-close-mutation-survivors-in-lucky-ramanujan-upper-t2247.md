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

About five minutes remain. Corrected mutant baseline passed (33 seconds); individual mutants are running. Keeping the tested implementation unchanged while the compact gate completes.
- 2026-10-21T03:57:21Z launcher: no commit for 60 s; 581 s elapsed. Files changed so far: docs/worker-notes/pc1-mpu-close-mutation-survivors-in-lucky-ramanujan-upper-t2247.md, src/real/mod.rs, tests/mutants_estimates.rs.

Mutation debug log confirms caught cases while buffered summary waits to flush. Compact target is about 573 MiB; quota blocker resolved. Gate has reached the full test suite, currently count compatibility tests.
Boundary classification of the 13 requested cases:
- harmreal < to ==: numerically equivalent in its u64 asymptotic domain; now killed by checking the actual correction iterator stops consuming terms.
- lucky n < 7 to <=: changes estimate at 7 from 3 to 2; public small-input bounds use independent formulas.
- lucky /305 to *305: negative estimate factor becomes zero; propagated lower bound remains valid but non-tight.
- lucky <=10000000 to >: at 1000001 the lower estimate decreases from 70936 to 70861; no lower-bound violation there.
- lucky /304 to *304: likewise zero, a valid but useless lower bound at the checked transition.
- lucky <=100000000 to >: at 10000001 the lower estimate decreases from 602729 to 600795; no lower-bound violation there.
- Axler >3999654659 to >=: changes the coefficient at equality; exact R_3999654659 was not computed, so the test enforces MPU's threshold without claiming a demonstrated bound violation.
- Axler delete minus on -2.0884: increases the Axler index and loosens its upper estimate; regression verifies MPU's intended coefficient.
- nth_prime_upper <15360 to <=: both branches remain above the exact prime; the test enforces MPU's selected upper estimate (168432).
- almost-prime >= span to <: synthetic plateau test catches the wrong retreat policy; this is interpolation progress, not an approximate bound contract.
- almost-prime guess <= min to >: synthetic plateau test catches the wrong bracket side.
- almost-prime max - 19999 to +: trace test catches an estimate outside the bracket.
- almost-prime max - 19999 to /: trace test catches the wrong retreat and resulting trace.
No incorrect unmutated bound was established; no numerical algorithm fix was warranted.

Mutation debug outcomes confirm all 13 requested mutants caught, zero missed. Detailed boundary classifications recorded above. Waiting for the final gate result; no READY marking.

Final cargo-mutants result: 13 mutants tested in 4 minutes, 13 caught, exit 0. git diff --check passed. Compact gate remains in the full-suite count oracle; all earlier stages (fmt, four clippy configurations, MSRV build) passed.

The full gate suite passed the long count oracle and advanced into family compatibility tests. Mutation confirmation is complete; full gate final status is still pending.
- 2026-10-21T04:00:55Z launcher: about 60 s to the time limit of 14 minutes; 795 s elapsed. Files changed so far: docs/worker-notes/pc1-mpu-close-mutation-survivors-in-lucky-ramanujan-upper-t2247.md, src/real/mod.rs, tests/mutants_estimates.rs.
