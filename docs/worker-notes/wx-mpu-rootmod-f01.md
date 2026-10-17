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
