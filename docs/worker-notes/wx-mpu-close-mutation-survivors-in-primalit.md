Read PLAN sections 5–8 and the named mutation sites before edits.
Add deterministic regression cases in tests/mutants_proving_random.rs.
Check certificate bounds and seeded random boundaries against the reference.
Run focused tests, the offline gate, and a targeted mutation rerun where available.
The reference matches the existing arithmetic. Exact random rejection boundaries need controlled draws; private test hooks will retain the public stream behavior.
Added private draw adapters and tests for the rejected tail, strict Kalai acceptance, sextic twist orders, and asymmetric zero polynomial operands. Focused unit tests are compiling.
The first focused build found that shared tests need a crate alias when included as unit tests. Added the alias and a controlled Miller–Rabin base-zero regression; focused tests are rerunning.
- 2026-10-21T01:11:04Z launcher: no commit for 61 s; 273 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-close-mutation-survivors-in-primalit.md, src/bigint/proving.rs, src/bigint/random.rs, tests/mutants_proving_random.rs.
Nine focused unit tests and the integration test passed. The gate found a missing crate doc on the shared test file; changed its loading to a path module so the required crate doc is valid. No arithmetic bug has been found.
