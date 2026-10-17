Port the four modular root APIs from MPU rootmod.c in separate files.
Reuse the existing modular arithmetic and factorization dependencies.
Add offline compatibility and algebraic tests, then run the gate.
The task forbids fixture, tooling, STATUS.md, and git metadata edits.
Read MPU square-root, generalized Tonelli-Shanks, Hensel lift, and CRT paths; implementing without new dependencies.
Implemented all four APIs and MPU helper paths. Next: fixture and exhaustive small-modulus tests, then lint.
Focused tests started; added oracle coverage, exhaustive small moduli, randomized laws, root limits, and concurrent repeat calls. Gate remains.
Focused test build is still running. Gate reached clippy and is waiting for the build lock; no network commands were used.
Review: APIs have no shared state; invalid modulus and repeated concurrent calls are covered. Offline compilation remains pending.
All four oracle tests and all five property/regression tests passed. Gate found only doc/lint issues; fixed bounded midpoint expressions, fallible small-table indices, and test crate docs.
Library lint is clean. Fixed benchmark semicolons and added full-word boundary cases and large root-count rejection.
Updated assertions for the installed pedantic clippy version. Full offline suite is running; gate will be rerun after these lint-only fixes.
Full-word regression cases added; final suite and gate are running offline. Checking that shared modular test files retain prior coverage.
The checkout has no origin/dev ref, so comparison against that ref is unavailable; using local file history to verify scope.
Local history revealed pre-existing sibling tests in the shared modular files. Restored them verbatim and placed the new root tests in nested modules to retain all coverage.
Restored all sibling oracle and property tests. New comments name signed fixture examples that the u64 API cannot accept.
Full suite compilation still holds the build lock. Preserved sibling tests; no algorithm failures have appeared in the completed focused tests.
Waiting on the full offline suite and gate. The test wrapper buffers its output until completion, so current full-suite progress is unavailable.
About five minutes remain. Gate has advanced past compilation and is producing test output; awaiting the final result.
Gate printed == gate: green: fmt, pedantic clippy, Rust 1.85 offline build, all tests, runnable docs, and documentation passed. Waiting for the separately requested tools/test.sh invocation.
Finished: focused oracle/property tests, the full offline tools/test.sh run, and tools/gate.sh all passed. Gate printed == gate: green. Sibling tests are preserved; signed/bigint cases remain documented API skips. No fixtures, tools, STATUS.md, dependencies, or git metadata were edited.
