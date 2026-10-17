Implement the four native random prime APIs from MPU 0.75.
Read PLAN sections 5–8 and the named C reference before editing.
Keep fixture, tooling, crate root, and generated STATUS unchanged per task constraints.
Use existing random state and primality APIs, then add compatibility, property, and benchmark coverage.
Confirmed existing thread-local CSPRNG and seeded fixtures. Native C uses rejection sampling and small nth-prime tables; no new dependency is needed.
Implemented all four functions in separate files with module exports and runnable docs. Added fixture dispatch, size/range laws, reseeding and concurrent replay tests, plus hot-function benchmarks. Focused offline tests are running.
