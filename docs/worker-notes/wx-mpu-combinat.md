Reassessed the revised combinatorics task and read PLAN.md sections 5–8.
The named C reference files are absent, so faithful algorithm and table verification is blocked.
Implement the independently verifiable factorial subset and use the existing oracle fixtures offline.
Do not modify tools, fixtures, crate root, dependencies, or git metadata.
Implemented five checked factorial functions; comments explicitly mark missing C source verification. Next: oracle and boundary tests.
Added oracle checks, algebraic laws, overflow/zero-step regressions, repeated/concurrent calls, and criterion benchmarks for the five functions.
Focused offline tests could not compile: criterion is absent from the local Cargo cache. No network retry is permitted. Format and gate checks follow.
STATUS ticks identify the five implemented functions; C algorithm fidelity remains unverified because reference/mpu is absent.
Gate passed formatting, then stopped at clippy dependency resolution: criterion is not cached. MSRV build, tests, and docs were not reached; gate is not green.
Final: five factorial APIs and their tests/benchmarks are written. Remaining: the other 23 APIs, their tests/benchmarks/status ticks, source verification against the missing C submodule, and all executable validation. Tools and fixtures wer <<cut 59 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 35 bytes)
