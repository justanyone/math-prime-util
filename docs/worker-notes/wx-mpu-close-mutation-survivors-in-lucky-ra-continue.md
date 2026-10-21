Continue the existing estimate mutation tests on this branch.
Review each reported survivor and distinguish bound validity from tightness.
Run the focused tests, the offline mutation sweep, and the repository gate.
Keep changes limited to the task paths and record blockers here.
Initial five tests passed. Gate found one narrowing cast; changed k to u8 and used u32::from. Added large lucky fit boundaries and the harmonic asymptotic branch.
Seven focused tests pass. The lucky <7 mutant is unreachable through public callers; harmonic termination is below f64 precision. Added Axler indices at the tail threshold and n=7140 (index 15360).
Nine focused tests pass. Clippy default and bigint pass. Added precise Axler boundary checks; mutation run still building its earlier snapshot.
Classification: lucky simple_approx n<7 is unreachable through public APIs (small tables; simple_lower calls it only above 9000). Its comparison mutant cannot be killed by a public API test.
Classification: harmreal < to == adds terms below 1e-44 to an accumulated correction of at least about 1e-40, or to a much larger correction. These additions do not change the returned f64 at tested boundaries.
The four interpolation mutations alter fallback guesses. Exact counting and final traversal can still recover the exact nth value. A surviving fallback mutation alone does not prove the returned result is wrong.
Mutation baseline failed before testing any mutant: copied target plus all test builds exceeded the sandbox disk quota. Retry with no target copy, no debug info, no incremental state, and only the new integration-test target.
Gate has passed fmt, all four clippy feature sets, and the MSRV build; its all-features test stage is running.
Retry now runs offline with debug/incremental output disabled. No production source has changed; the tests preserve the C algorithms and do not add observability hooks solely to kill equivalent mutants.
