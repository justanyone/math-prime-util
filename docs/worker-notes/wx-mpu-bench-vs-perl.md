Implement the revised tooling task in tools/ only, plus this required log.
Use benches/int.rs as the workload source. Leave src/, fixtures, and STATUS.md unchanged.
Compare Criterion samples with MPU 0.75 XS on this host. Report the 1.2x bar only.
Run offline focused checks and the repository gate. Record missing local prerequisites.
MPU is not installed on this host. Focused tests are running offline. Use isolated Criterion output per launch and reject missing XS before timing.
Checked the existing int tests and benches. The reference submodule is empty; the comparison must use the installed XS module without network bootstrap.
Added the Perl driver and a Cargo wrapper. Private manifests enable Criterion without changing Cargo.toml; private sample paths prevent duplicate launches from mixing results.
Added all nine existing benchmark IDs, including both root degrees. XS is missing locally; add offline regression checks with fake processes, then verify the real Rust runner and gate.
Compat int: 12 passed. Property int: 9 passed. The gate is running. Added regression cases for invalid samples, exact inputs and counts, and concurrent sample isolation.
Tooling checks found a timing-sensitive fake-data assertion. Increased fake Rust times so the report-only threshold case is deterministic. All fixture and property checks remain unchanged.
The explicit tooling regression suite passes through tools/test.sh. It also checks u64 input precision, backend/version refusal, failed Cargo runs, and cleanup. Final gate is green.
