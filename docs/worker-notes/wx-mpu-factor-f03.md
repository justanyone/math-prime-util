Port the four bounded factor methods from MPU 0.75 with their native defaults.
Keep each public function in its own source file and reuse existing arithmetic.
Check committed factor fixtures and algebraic properties without Perl or network.
Leave generated status, tools, and fixtures unchanged under the task-specific rules.
Read the C kernels and XS defaults. Added rho, Brent, and shared wrapper arithmetic; next port p-1 and p+1 and test fixtures.
Added p-1 stage-one recovery and stage-two gap cache, p+1 Lucas powering, fixture tests, and product/repeat/concurrency laws. Running focused tests next.
First focused run: rho, p-1, and p+1 fixtures pass. Brent fixture parsing hit an explicit undef bound; handling default arguments next.
Correction: the fixture contains 10foo, not undef. Named that typed-API skip explicitly; first gate found benchmark semicolons, now fixed.
