Port the four assigned factor functions from MPU 0.75.
Read PLAN sections 5–8 and the required scripts before editing code.
Use the existing factor fixture without changing tools or fixtures.
Add compatibility, algebraic tests, and benchmarks; run offline checks.
The factor module has no factor/factor_exp yet; inspecting the C rho helper for a private dependency fallback.
Added all four APIs and fixture tests. Chebyshev follows the C bounds and Q=1 Lucas ladder with unsigned modular arithmetic.
Focused compatibility test is running offline. Added algebraic, edge, repeated-call, and thread tests plus benchmarks.
All 4 compatibility and 5 property/edge/thread tests pass. Replaced the provisional rho loop with the MPU 64-step batching and retries; gate is running.
Gate found only benchmark style and empty-assert lint errors. Fixed those; no functional failure.
