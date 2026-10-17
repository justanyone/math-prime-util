Implement factor, factor_exp, divisors, and trial_factor from MPU 0.75.
Read PLAN sections 5–8 and the prescribed test and gate scripts.
Keep fixtures, tooling, crate root, and generated STATUS unchanged per revised scope.
Add compatibility, algebraic tests, and benchmarks; validate offline.
Confirmed fixture argument shapes and C zero/one behavior. Other factor-method rows are absent, so use private C HOLF/Brent helpers for the split loop.
Implemented the four APIs, compatibility tests, property laws, concurrency/repeated-call checks, and benchmarks. Running the existing offline fixture tests next.
Added helper-path checks for wheel alignment, round exhaustion, and split methods. Started the offline gate; fixture and property test build is still running.
The first gate found three lint issues; corrected narrow lint annotations and checked conversion. Replaced small-prime discovery with MPU's fixed 7–2011 prime table for matching trial bounds and faster calls.
All four oracle tests and all four property/boundary tests passed offline. Fixed the newer clippy empty-assert lint. Final gate will also exercise the private helper tests and runnable examples.
Final validation window: fixture tests passed; gate rerun includes fixed-table trial division and extra Brent failure-path tests. The C split dispatcher still lacks SQUFOF, p−1, rho, Chebyshev, and ECM sibling methods here; private HOLF/Bre <<cut 36 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 12 bytes)
The gate caught an incorrect helper-test expectation: MPU extracts 3 and 5 before applying its square-root stop, leaving 2. Corrected the test to [3, 5, 2]; rerunning the gate.
