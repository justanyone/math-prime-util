Reassessed the revised combinatorics task and read PLAN.md sections 5–8.
The named C reference files are absent, so faithful algorithm and table verification is blocked.
Implement the independently verifiable factorial subset and use the existing oracle fixtures offline.
Do not modify tools, fixtures, crate root, dependencies, or git metadata.
Implemented five checked factorial functions; comments explicitly mark missing C source verification. Next: oracle and boundary tests.
Added oracle checks, algebraic laws, overflow/zero-step regressions, repeated/concurrent calls, and criterion benchmarks for the five functions.
Focused offline tests could not compile: criterion is absent from the local Cargo cache. No network retry is permitted. Format and gate checks follow.
STATUS ticks identify the five implemented functions; C algorithm fidelity remains unverified because reference/mpu is absent.
