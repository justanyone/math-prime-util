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
Caught candidates: prime_factors 52:14 (`<` to `>` or `<=`), Lehman 49:52 (`%` to `/`), Lehman 56:14 (`*` to `+` or `/`), p-1 42:15 (`+` to `-` or `*`), HOLF 34:33 (`>>` to `<<`). Direct regression tests cover these paths. Mutation verifica <<cut 16 chars by kit>>
Equivalent: prime_factors 52:14 (`<` to `==`): after small-prime stripping, the pending value is 1 or is greater than 43; it cannot be 0 or 2.
Equivalent: Lehman 49:52 (`%` to `+`): the residue is reduced modulo four at its only use, so both give the same b.
Equivalent: p-1 56:35 (`+` to `-` or `*`): replay can repeat primes at or below save_q. Their full prime powers are already in save_a. Replay occurs only when the final gcd is n. Every factor order then divides the final exponent. Repeating <<cut 104 chars by kit>>
Still open: prime_factors 64:15 retry updates; factor 108:32 round schedule; factor 161:11 fallback updates. No witness or equivalence proof found.
Still open: the unnamed remainder of the roughly 35 survivors. The task supplies no mutation operators or report for them; no blanket equivalence claim is made.
The requested aragog run needs remote access, which this task forbids. Local run uses nice 19, -j 2, --re and offline library tests on wowbagger.
No production code, dependency, fixture or build tooling changed. STATUS.md already checks all named functions.
State and concurrency: these tests call deterministic local splitters. There is no launch or shared-state path in the changed code. Empty and small helper inputs have direct assertions.

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 71 bytes)
HOLF regression passes. Stopped a slow target-copy mutation attempt and restarted without copying target artifacts. Local mutation results are not yet available.
