Port the named totient and Moebius functions from the read-only MPU source.
Reuse the existing integer and factoring APIs after checking their definitions.
Use the committed arith oracle without changing tools, fixtures, or crate exports.
Add compatibility, algebraic, and benchmark coverage, then run the offline gate.
Implemented scalar functions, range sieves, and cached summatory recurrences. The factor module is empty, so arithmetic has a private native factoring helper.
Added the 14 oracle tests, range and algebraic properties, concurrency checks, and benchmarks. Focused compatibility tests and the gate are running offline.
Property tests pass. The oracle run passed 10 functions; four tests only failed on signed arguments, now explicitly skipped as outside the unsigned API. Removed a self-comparison from valuation testing.
About five minutes remain. Ported the private Pollard rho helper to the reference batching and backtracking method; fixed the remaining Clippy formatting findings. Next: rerun the gate.
Gate reached the full tests: 13 of 14 arithmetic oracle tests pass. A signed Dedekind psi case lacked its intended skip; fixed the test adapter and rerunning.
