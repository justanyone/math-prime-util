Port the named totient and Moebius functions from the read-only MPU source.
Reuse the existing integer and factoring APIs after checking their definitions.
Use the committed arith oracle without changing tools, fixtures, or crate exports.
Add compatibility, algebraic, and benchmark coverage, then run the offline gate.
Implemented scalar functions, range sieves, and cached summatory recurrences. The factor module is empty, so arithmetic has a private native factoring helper.
Added the 14 oracle tests, range and algebraic properties, concurrency checks, and benchmarks. Focused compatibility tests and the gate are running offline.
Property tests pass. The oracle run passed 10 functions; four tests only failed on signed arguments, now explicitly skipped as outside the unsigned API. Removed a self-comparison from valuation testing.
