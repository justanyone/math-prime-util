Port sopfr, sopf, kronecker, and chinese into separate arithmetic files.
First compare the existing definitions with MPU 0.75 and reuse existing fixtures.
Add boundary and algebraic tests, then run focused tests and the offline gate.
Keep generated status, fixtures, tooling, and unrelated functions unchanged.
Existing APIs and oracle tests are present; split the four functions and restore the C small-input bounds. Chinese needs its own C fallback rather than delegating to chinese2.
Split implementation is written. Added C Shell-sort gaps, extended Euclid, and coprime overflow fallback for chinese; focused oracle/property tests are running offline.
The first compile found sibling residue functions also use the unsigned Kronecker helper. Kept those shared private helpers in mod.rs and imported them into the new signed wrapper.
Added factor-sum additivity, Kronecker multiplicativity, CRT reconstruction and permutation laws, native overflow regressions, repeated calls, concurrent calls, and four benchmarks. Gate is waiting for the focused build lock.
The regression now checks the C overflow fallback rejects wide noncoprime moduli that the prior chinese2 delegation accepted. Offline focused tests and gate are still running.
Focused offline validation passed: 31 arithmetic compatibility tests and 23 arithmetic property/regression tests. Gate formatting and pedantic clippy passed; MSRV build is running.
Rust 1.85 build passed; full gate tests are compiling. The checkout has no origin/dev ref, so a base-ref diff review is unavailable; no network or git writes were attempted.
The full offline suite is running its unit tests; formatting, clippy, MSRV, and arithmetic tests remain green. No fixtures, tooling, dependencies, or generated status changed.
Finished: tools/gate.sh printed == gate: green, including formatting, pedantic clippy, Rust 1.85, the full offline test suite, 435 doctests, and documentation. Focused arithmetic tests passed 54 cases; no requested implementation remains.
