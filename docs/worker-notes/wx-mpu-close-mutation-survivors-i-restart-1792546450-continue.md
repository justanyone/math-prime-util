Continue the estimate mutation task from the existing integration tests.
Read PLAN sections 5–8 and inspect the named private helpers before editing tests.
Add regression cases in tests/mutants_estimates.rs and keep production changes minimal.
Run offline focused tests, the available mutation runner, and the project gate.
Inspected the C fits and interpolation: several changes only affect search brackets; add public output checks and measure survivors before claiming kills.
Focused tests now cover independent lucky inversion, exact almost-prime ranks, and the Ramanujan coefficient threshold. Initial lucky equality required one unit of tolerance for the inverse convention.
Nine focused integration tests pass. Started the exact 13-mutant offline sweep on the four requested files and the offline project gate.
Confirmed adjacent Ramanujan indices 132 and 133 map to prime indices 305 and 307, so the private n=306 comparison cannot affect these public boundary calls. Added regression checks for that transition.
The harmonic comparison mutant survives as expected. Added separate large-rank almost-prime cases to reach interpolation corrections; checking their runtime before the final sweep.
Mutation sweep stopped after five reported survivors: disk quota exceeded while writing its log. Formatting, all four Clippy configurations, and MSRV passed; gate tests are running. Public lucky inverse checks survive the four fit mutations because the search recovers the same result.
All eleven integration tests pass, including ranks 10 million and 100 million (2.4 seconds). Restarted the exact 13-mutant sweep with one job, no target copy, no incremental cache, and scratch files inside the worktree to avoid the temporary-filesystem quota.
