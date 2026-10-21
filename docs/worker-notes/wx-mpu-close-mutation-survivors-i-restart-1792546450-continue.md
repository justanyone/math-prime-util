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
Boundary review: lucky n=7 is shielded by the small public table and simple_lower n<=9000. The four larger lucky fits only seed valid inverse-search brackets on sampled inputs. Removing the negative Ramanujan tail coefficient raises the bound, so that mutant is looser rather than a demonstrated underbound.
Boundary classification for the 13 requested mutations:
- real/mod.rs:763 (< to ==): extra terms below 1e-44 cannot change the f64 sum, whose leading correction at the first asymptotic input exceeds 1e-8. This is numerically equivalent, not a bound failure.
- lucky.rs:327 (< to <=): only n=7 differs inside the private helper; public small tables and simple_lower bypass it. Public bounds remain unchanged there.
- lucky.rs:337,338,339,340: the changed fits seed the lower bracket. Independent bisection checks both sides of 10 million and 100 million and at 4 billion. The first sweep reports the same public estimates for all four mutations. These cas <<cut 43 chars by kit>>
- nth_ramanujan_prime_upper.rs:121 (> to >=): changes the coefficient only at 3,999,654,659; pin the C result there and on both sides. The stronger mutated bound has not been proved invalid by an exact value at this very large rank.
- nth_ramanujan_prime_upper.rs:124 (delete minus): the larger coefficient increases the Axler index and loosens its prime upper bound. Pin C parity despite the bound remaining conservative.
- nth_ramanujan_prime_upper.rs:146 (< to <=): the private index 306 would use the exact prime rather than an upper bound. Adjacent public ranks 132 and 133 skip from index 305 to 307; those bounds remain valid.
- nth_almost_prime.rs:155,157,160 (two arithmetic changes): these control guess correction, not an approximation returned to the caller. Exact count and membership checks cover ranks through 100 million; record mutation outcomes separately  <<cut 61 chars by kit>>
No production defect has been established. The all-13-killed criterion includes equivalent or public-unreachable mutations and cannot be honestly claimed from public integration tests alone.

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 56 bytes)
