Port the four factor predicates from MPU 0.75 in separate source files.
Keep shared module edits minimal; leave generated status and fixtures unchanged.
Use existing offline fixtures and add algebraic property tests and benchmarks.
Run focused tests and the repository gate; record any unavailable dependencies.
The factor module has no prerequisite factoring API; predicate fallbacks will use private support and retain the C filters and bounds.
Implemented predicates, private C rho/trial support, oracle tests, independent counts, concurrency checks, and benchmarks; running focused tests next.
Focused compile found Option-returning modular helpers; corrected handling for the nonzero internal modulus and rerunning.
Oracle includes bigint arguments beyond u64 (including k=10^30); documented native-API skips. All representable omega fixtures passed.
Native oracle and all five property tests pass; added direct tests for rho failure, backtracking support, and exact trial fallback. Fixing copied-table lint.
Gate passed production-code clippy; adjusted two test assertions for the installed toolchain’s newer assert_is_empty lint. Running the final gate.
Finished: native oracle, algebraic, boundary, concurrent, and private factoring tests pass; tools/gate.sh printed == gate: green offline. Cross-platform CI and coverage measurement remain for the launcher; no tools, fixtures, status, dependencies, or git metadata were changed.
