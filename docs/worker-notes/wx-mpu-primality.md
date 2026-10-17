Reassess the existing primality port against MPU 0.75 and its fixtures.
Keep changes in the named task paths and use the existing Montgomery context.
Run focused offline tests, fix concrete gaps, and run the required gate.
Leave random Miller-Rabin deferred until the random module is available.
Existing 13 compat tests and 4 property tests pass offline. Missing functions require additional reference ports; fixture and tooling files remain unchanged.
Ported all three Frobenius methods and native explicit parameters; added fixture checks, with named skips for bigint fallback parameters.
First Frobenius run found an oracle arity error (lone P); named that compile-time Rust API exclusion. Added parameter, square, prime, concurrency regressions and benchmarks.
All 16 compat tests and 8 property/regression tests pass. Full offline gate started; remaining scope is Perrin, Catalan, Chen, delicate prime, and the explicitly deferred random Miller-Rabin API.
Gate caught manual ilog2; replaced it with the standard method. Preserved the C Underwood x=0 fast path. About five minutes of the session budget remain; finish validation and record remaining work.
NOTES: Perrin, Catalan, Chen, and delicate-prime ports and their fixture/property tests remain unwritten. Random Miller-Rabin remains deferred and unchecked in STATUS.md. No fixture, tooling, dependency, or crate-root edits were made.
Final: tools/gate.sh printed == gate: green (fmt, pedantic clippy, MSRV 1.85 build, all tests, doctests, docs), entirely offline. Focused tests passed 16 compat and 8 property/regression tests. Coverage and other OS/architecture jobs were not measured here. Result is PARTIAL for the remaining ports listed above; the launcher owns commits.
