Implement factor, factor_exp, divisors, and trial_factor from MPU 0.75.
Read PLAN sections 5–8 and the prescribed test and gate scripts.
Keep fixtures, tooling, crate root, and generated STATUS unchanged per revised scope.
Add compatibility, algebraic tests, and benchmarks; validate offline.
Confirmed fixture argument shapes and C zero/one behavior. Other factor-method rows are absent, so use private C HOLF/Brent helpers for the split loop.
Implemented the four APIs, compatibility tests, property laws, concurrency/repeated-call checks, and benchmarks. Running the existing offline fixture tests next.
Added helper-path checks for wheel alignment, round exhaustion, and split methods. Started the offline gate; fixture and property test build is still running.
