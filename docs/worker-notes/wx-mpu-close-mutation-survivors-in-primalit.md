Read PLAN sections 5–8 and the named mutation sites before edits.
Add deterministic regression cases in tests/mutants_proving_random.rs.
Check certificate bounds and seeded random boundaries against the reference.
Run focused tests, the offline gate, and a targeted mutation rerun where available.
The reference matches the existing arithmetic. Exact random rejection boundaries need controlled draws; private test hooks will retain the public stream behavior.
Added private draw adapters and tests for the rejected tail, strict Kalai acceptance, sextic twist orders, and asymmetric zero polynomial operands. Focused unit tests are compiling.
The first focused build found that shared tests need a crate alias when included as unit tests. Added the alias and a controlled Miller–Rabin base-zero regression; focused tests are rerunning.
- 2026-10-21T01:11:04Z launcher: no commit for 61 s; 273 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-close-mutation-survivors-in-primalit.md, src/bigint/proving.rs, src/bigint/random.rs, tests/mutants_proving_random.rs.
Nine focused unit tests and the integration test passed. The gate found a missing crate doc on the shared test file; changed its loading to a path module so the required crate doc is valid. No arithmetic bug has been found.
The mutation baseline failed after exhausting the filesystem quota. Clippy found duplicate module loading and an unnecessary private Option; fixing both before the final gate.
- 2026-10-21T01:13:36Z launcher: no commit for 61 s; 425 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-close-mutation-survivors-in-primalit.md, src/bigint/proving.rs, src/bigint/random.rs, tests/mutants_proving_random.rs.
Focused tests pass after the clippy fixes. Restarted the exact-site mutation run with nice 15, two jobs, offline Cargo, and a shared target directory to avoid another copied-target quota failure.
About five minutes remain. All four clippy feature sets passed; MSRV and full tests are running. The corrected mutation rerun selects all fourteen sites. Strengthened BLS trial-factor output and zero-product early-return tests.
Added the BLS prime 1,028,456,449 with n−1 = 2^16 × 3 × 5231. Its certificate must retain the large cofactor after trial division, distinguishing both BLS survivors. Full validation is still waiting on Cargo build locks.
The fourteen-site mutation baseline now reports Cargo cache error 8 (attempt to write a readonly database), then build/artifact locks. No mutant outcome is confirmed. All requested regression cases are written; final gate and whole-suite jo <<cut 18 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed -6 bytes)
The final focused run passed all eight tests, including the added BLS cofactor and zero-polynomial cases. The mutation baseline builds every integration binary by default; limiting its Cargo build to --lib for the remaining validation attem <<cut 3 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed -20 bytes)
The broad mutation baseline did eventually pass (81 s build, 36 s test). Interrupted it and restarted all fourteen targets with --lib builds and a skipped, already-passing baseline. The gate has advanced past MSRV into full tests.
Added a controlled last-bin prime test: seeded samples almost never choose the last of 2^33 bins. The helper keeps public sampling unchanged and checks the exact residual width.
The fourteen-site run reports 14 caught in 60 seconds. Auditing the mutation test logs before accepting that result. The final focused suite now passes nine tests, including the explicit last-bin case.
Audit rejected the apparent 14 catches: duplicated --lib made the test command fail. Restarting with --lib only in Cargo arguments and a real baseline; the earlier counts are invalid.
Final validation uses a corrected Cargo invocation and real baseline. No existing test file was edited, no dependency was added, and STATUS entries were already checked. No production arithmetic bug was found; changes expose deterministic d <<cut 34 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 10 bytes)
