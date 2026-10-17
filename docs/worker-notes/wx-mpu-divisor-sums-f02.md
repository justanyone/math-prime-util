Port sopfr, sopf, kronecker, and chinese into separate arithmetic files.
First compare the existing definitions with MPU 0.75 and reuse existing fixtures.
Add boundary and algebraic tests, then run focused tests and the offline gate.
Keep generated status, fixtures, tooling, and unrelated functions unchanged.
Existing APIs and oracle tests are present; split the four functions and restore the C small-input bounds. Chinese needs its own C fallback rather than delegating to chinese2.
Split implementation is written. Added C Shell-sort gaps, extended Euclid, and coprime overflow fallback for chinese; focused oracle/property tests are running offline.
The first compile found sibling residue functions also use the unsigned Kronecker helper. Kept those shared private helpers in mod.rs and imported them into the new signed wrapper.
Added factor-sum additivity, Kronecker multiplicativity, CRT reconstruction and permutation laws, native overflow regressions, repeated calls, concurrent calls, and four benchmarks. Gate is waiting for the focused build lock.
