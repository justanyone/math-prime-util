Implement the four native random prime APIs from MPU 0.75.
Read PLAN sections 5–8 and the named C reference before editing.
Keep fixture, tooling, crate root, and generated STATUS unchanged per task constraints.
Use existing random state and primality APIs, then add compatibility, property, and benchmark coverage.
Confirmed existing thread-local CSPRNG and seeded fixtures. Native C uses rejection sampling and small nth-prime tables; no new dependency is needed.
Implemented all four functions in separate files with module exports and runnable docs. Added fixture dispatch, size/range laws, reseeding and concurrent replay tests, plus hot-function benchmarks. Focused offline tests are running.
Focused offline compatibility and property tests passed: 34 tests total, including all supported native oracle cases. The full gate is running; no fixture, dependency, or tooling changes were needed.
The gate caught four benchmark closure semicolons required by pedantic clippy. Fixed those statements and restarted the gate.
Added explicit regression coverage that empty, singleton, and unsupported prime requests preserve stream position. Named the fixture cases skipped by the native API.
The gate passed fmt, pedantic clippy, and MSRV 1.85 build, and reached full-suite tests. Started the required tools/test.sh whole-suite run after the final regression addition.
The first full gate printed == gate: green. Re-running the gate against the final test additions while the required whole-suite test wrapper finishes. Base ref origin/dev is absent locally; no git writes were attempted.
