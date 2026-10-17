Port the four named almost-prime functions from MPU 0.75.
Check existing count and interpolation dependencies before choosing shared helpers.
Keep fixtures, tools, STATUS.md, and the crate root unchanged as required by the task.
Run focused offline tests and the gate; record any missing dependencies.
Dependency review: count bounds and inverse interpolation need private ports; semiprime APIs already exist.
- 2026-10-17T18:45:34Z launcher: no commit for 60 s; 122 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-almost-omega-f02.md.
Implemented bounds tables, reductions, count inversion, and the four public functions. Next add fixture and property coverage.
Added native oracle checks, property tests, concurrency checks, and benchmarks. Focused tests are compiling offline.
Focused tests and gate are waiting on the Cargo build lock. Next inspect failures once compilation finishes.
