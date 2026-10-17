Reassess the existing primality port against MPU 0.75 and its fixtures.
Keep changes in the named task paths and use the existing Montgomery context.
Run focused offline tests, fix concrete gaps, and run the required gate.
Leave random Miller-Rabin deferred until the random module is available.
Existing 13 compat tests and 4 property tests pass offline. Missing functions require additional reference ports; fixture and tooling files remain unchanged.
Ported all three Frobenius methods and native explicit parameters; added fixture checks, with named skips for bigint fallback parameters.
First Frobenius run found an oracle arity error (lone P); named that compile-time Rust API exclusion. Added parameter, square, prime, concurrency regressions and benchmarks.
All 16 compat tests and 8 property/regression tests pass. Full offline gate started; remaining scope is Perrin, Catalan, Chen, delicate prime, and the explicitly deferred random Miller-Rabin API.
