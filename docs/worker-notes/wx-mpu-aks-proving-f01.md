Reassess the existing AKS and native proving functions against MPU 0.75.
Keep the verifier and sibling work intact. Split the requested public functions into their own files.
Check oracle compatibility, algebraic laws, and offline gate results.
Do not change fixtures, tools, crate dependencies, or generated status.
Confirmed native XS proving aliases is_prime and _small_cert permits all u64 values; split the three proving APIs into individual modules.

Reassessment: the AKS bounds and Lanczos table match IMPL_BERN41; focused oracle and property tests are compiling offline.
Ported the C degree-limited multiplication and symmetric square paths, including bounded u128 accumulation; retained Bernstein constants and search bounds.
Added direct polynomial regressions for sparse, dense, zero, native, and u64-boundary inputs; extended certificate edge tests and all-four-function benchmarks.
Focused runs and the gate are active; the gate currently waits for the Cargo build lock. No dependency, fixture, verifier, or tooling changes were needed.
Added the product-compositeness law for all proving and certificate APIs, with unsigned input limits documented in the oracle tests.
Initial offline oracle and property runs passed (12 compatibility tests and 11 property tests); the final optimized implementation is now building for unit and lint checks.
All requested code is written. Native certificates keep the MPU text format; no bigint or ECPP work is needed for this slice. Final compatibility, property, benchmark-build, and gate checks are in progress.
The proving unit tests passed, and clippy completed with -D warnings. The gate has advanced to the offline MSRV build.
Final focused oracle/property tests passed, benchmark compilation passed, and Rust 1.85 build passed. Full-suite and gate test stages remain active.
- 2026-10-17T18:52:24Z launcher: no commit for 60 s; 366 s elapsed. Files changed so far: benches/primality.rs, docs/worker-notes/wx-mpu-aks-proving-f01.md, src/proving/is_aks_prime.rs, src/proving/is_provable_prime.rs, src/proving/is_provable_prime_with_cert.rs, src/proving/mod.rs, src/proving/prime_certificate.rs, tests/compat_primality.rs, tests/prop_primality.rs.
The full-suite run is compiling integration tests offline. All completed targeted checks remain green.
