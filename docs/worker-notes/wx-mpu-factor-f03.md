Port the four bounded factor methods from MPU 0.75 with their native defaults.
Keep each public function in its own source file and reuse existing arithmetic.
Check committed factor fixtures and algebraic properties without Perl or network.
Leave generated status, tools, and fixtures unchanged under the task-specific rules.
Read the C kernels and XS defaults. Added rho, Brent, and shared wrapper arithmetic; next port p-1 and p+1 and test fixtures.
Added p-1 stage-one recovery and stage-two gap cache, p+1 Lucas powering, fixture tests, and product/repeat/concurrency laws. Running focused tests next.
First focused run: rho, p-1, and p+1 fixtures pass. Brent fixture parsing hit an explicit undef bound; handling default arguments next.
Correction: the fixture contains 10foo, not undef. Named that typed-API skip explicitly; first gate found benchmark semicolons, now fixed.
All four fixture tests and all three property/edge/concurrency tests pass offline. The gate is running after the benchmark lint fix.
Gate printed green. Added direct retry, wide arithmetic, bound, recovery, and independent Lucas-recurrence tests to improve kernel coverage.
Direct tests exposed two incorrect expected p-1 outcomes; corrected them from the modular-order calculation (simultaneous smooth factors can remain unsplit). cargo-llvm-cov is absent, so coverage cannot be measured here.
Final: all four ports, runnable docs, fixture/property/unit tests, and benchmarks are written. Focused and full tools/test.sh runs pass; final tools/gate.sh prints green. Coverage measurement remains unavailable because cargo-llvm-cov is no <<cut 83 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 59 bytes)
