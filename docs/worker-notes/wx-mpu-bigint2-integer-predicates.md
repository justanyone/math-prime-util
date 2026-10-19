Implement bigint integer predicates from the MPU reference with existing bigint factors.
Add the bigint_predicates oracle generator, replay tests, and native agreement properties.
Keep native APIs unchanged and propagate factorization failures explicitly.
Run offline focused tests and the required gate; record unavailable checks here.

Read native signatures and C factor criteria; implement practical factor predicates first. Congruent-number, Pillai, and inverse-totient searches need feasibility review.

Host is wowbagger, not the requested aragog/wonko. Build-host constraint may block execution; oracle must stay offline.

Implemented 15 predicates, including inverse-totient recursion. All integer arguments use BigUint; factor errors remain explicit. STATUS entries were already checked.

Source generation needed correction after command text truncation. Corrected the two generated declarations; next add fixtures and tests.
