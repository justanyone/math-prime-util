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
At the tested large lucky fit transitions, the bounds still bracket the exact count. The initial estimate is a search bracket; a changed bracket can leave the final inverse result unchanged. No bound violation has been observed.
Mutation baseline passes after reducing build output. The harmreal termination mutant survives. Full gate tests continue; no source changes are justified by the current evidence.
Offline cargo-mutants completed: 13 tested, 3 caught, 10 missed. All three Ramanujan mutations are caught by the new Axler composition tests. No timeouts or unviable mutants.
Remaining: real/mod.rs:763 <=>==; lucky.rs:327 <=><=, :337 /=>*, :338 <= => >, :339 /=>*, :340 <= => >; nth_almost_prime.rs:155 >= => <, :157 <= => >, :160 -=>+ and -=>/.
Bound classification: deleting the Axler D minus sign raises the Axler index, so it loosens a valid upper bound. The exact-threshold D change lowers the bound; its actual Ramanujan-bound validity at this billion-scale index has not been independently proved. The 15360 switch changes the selected prime bound; the regression checks C-compatible branch selection.
The lucky fit mutations still bracket exact lucky counts at all five large fit-transition inputs tested. The four almost-prime fallback mutations still give the brute-force exact values tested. This does not establish equivalence at every u64 input.
The mutation check is complete (3 caught, 10 survived); all nine focused tests pass. The full gate is still in the existing prime_count oracle test. The task remains partial because ten requested killing tests are not achieved.
