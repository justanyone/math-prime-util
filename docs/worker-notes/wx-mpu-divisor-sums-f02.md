Port sopfr, sopf, kronecker, and chinese into separate arithmetic files.
First compare the existing definitions with MPU 0.75 and reuse existing fixtures.
Add boundary and algebraic tests, then run focused tests and the offline gate.
Keep generated status, fixtures, tooling, and unrelated functions unchanged.
Existing APIs and oracle tests are present; split the four functions and restore the C small-input bounds. Chinese needs its own C fallback rather than delegating to chinese2.
