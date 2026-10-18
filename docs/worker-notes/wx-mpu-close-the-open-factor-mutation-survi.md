Read PLAN.md sections 5 to 8 and the named factor paths.
Add test-only cases for the open factor mutation survivors.
Record caught mutations and equivalent mutations with reasons in this PR text.
Use offline checks. The requested aragog run is unavailable without remote access.
Added direct prime_factors inputs and Lehman odd/square cases; host is wowbagger, not aragog. Next run focused tests and local mutations.
Focused tests are compiling offline. All named public functions are already checked in STATUS.md; no checklist edit is needed.
Listed the exact mutations. Started a local nice-19, two-job run with --re and offline library tests. Added p-1 replay cases.
Independent modular checks found a Lehman division witness. The addition mutation in its residue is equivalent modulo four; focused compilation is still running.
Corrected a Lehman expectation after the focused test failed. Stopped the mutation run to avoid counting that failure as a kill; p-1 replay test passes.
Found p-1 checkpoint witnesses with three prime factors. Two-factor inputs hid the gap because replay selected the same divisor. Added exact split assertions.
Focused library tests pass: 71 tests. The gate has passed fmt, clippy and the MSRV build and is running the full suite offline.
Found a HOLF shift witness in the multiplier-30 interval. Added a two-round private splitter assertion. Brent retry searches found no witness; those mutations remain open.

PR text draft (line numbers refer to the original source):
Caught candidates: prime_factors 52:14 (`<` to `==` or `>`), Lehman 49:52 (`%` to `/`), p-1 42:15 (`+` to `-` or `*`). Direct regression tests cover these paths. Mutation verifica <<cut 16 chars by kit>>
Equivalent: prime_factors 52:14 (`<` to `<=`): small-prime stripping removes every factor of 2. Pending values are 0, 1, or odd values above 43; no pending value is 2.
Equivalent: Lehman 49:52 (`%` to `+`): the residue is reduced modulo four at its only use, so both give the same b.
Equivalent: p-1 56:35 (`+` to `-` or `*`): replay can repeat primes at or below save_q. Their full prime powers are already in save_a. Replay occurs only when the final gcd is n. Every factor order then divides the final exponent. Repeating <<cut 104 chars by kit>>
Still open: prime_factors 64:15 retry updates; factor 108:32 round schedule; factor 161:11 fallback updates. No witness or equivalence proof found.
Still open: the unnamed remainder of the roughly 35 survivors. The task supplies no mutation operators or report for them; no blanket equivalence claim is made.
The requested aragog run needs remote access, which this task forbids. Local run uses nice 19, -j 2, --re and offline library tests on wowbagger.
No production code, dependency, fixture or build tooling changed. STATUS.md already checks all named functions.
State and concurrency: these tests call deterministic local splitters. There is no launch or shared-state path in the changed code. Empty and small helper inputs have direct assertions.

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 71 bytes)
HOLF regression passes. Stopped a slow target-copy mutation attempt and restarted without copying target artifacts. Local mutation results are not yet available.
Corrected the prime_factors equivalence note: <= is equivalent because pending values cannot be 2; == is caught by input 0. About five minutes remain; finishing validation and recording unresolved items.
Final fmt and clippy pass. Final gate is on the MSRV build. Mutants have copied the source and are building; no results are counted yet.
Validation continues offline. The original gate reports a long-running forsquarefreeint test; no failure has been reported. Retry and dispatcher survivors remain explicitly open.
Mutation results confirm dispatcher and fallback mutations remain missed. The first snapshot predates the HOLF test; rerunning that one mutation against the final source using cached dependencies.
The small-input mutations time out on the private factor tests, which catches their nontermination. The superseded gate was stopped; the final gate and sole tools/test.sh whole-suite run continue.
- 2026-10-18T15:46:18Z launcher: no commit for 60 s; 708 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-close-the-open-factor-mutation-survi.md, src/factor/holf_factor.rs, src/factor/is_carmichael.rs, src/factor/lehman_factor.rs, src/factor/pminus1_factor.rs.
The gate and full suite failed during concurrent builds after the HOLF rerun shared their target directory. Those results and the shared-target HOLF miss are invalid. Started isolated HOLF verification and a clean gate; no green gate is claimed.

Equivalent: Lehman 56:14 (`*` to `+` or `/`) removes the exact-square shortcut for odd cofactors above 5. If k*n is square, its first square k is odd. The interval then starts at twice its root and sees c = 0. Its gcd is gcd(2*root,n) = gcd(root,n), since n is odd. Both paths return the same pair.
Confirmed locally: the p-1 replay mutations survive, as predicted by the exponent argument. The dispatcher division mutation is caught by the existing tests; the addition mutation remains open.
- 2026-10-18T15:47:49Z launcher: no commit for 60 s; 798 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-close-the-open-factor-mutation-survi.md, src/factor/holf_factor.rs, src/factor/is_carmichael.rs, src/factor/lehman_factor.rs, src/factor/pminus1_factor.rs.
- 2026-10-18T15:47:49Z launcher: about 60 s to the time limit of 14 minutes; 798 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-close-the-open-factor-mutation-survi.md, src/factor/holf_factor.rs, src/factor/is_carmichael.rs, src/factor/lehman_factor.rs, src/factor/pminus1_factor.rs.

Final: removed the HOLF test because isolated verification showed it did not kill the mutation. Its model missed the overflow exit. HOLF 34:33 remains open. The local run caught four mutations, timed out on two named small-input mutations, and left the documented equivalents and unresolved retries alive. Two additional timeouts came from unrelated generated struct mutations and are excluded from task results.
Final validation: 71 focused factor tests passed before the removed HOLF test. Fmt, clippy and MSRV passed. The full suite failed during shared-target interference; the clean gate has not finished and has not printed green. No full-suite pass is claimed. Aragog was not accessed. This task remains partial.
