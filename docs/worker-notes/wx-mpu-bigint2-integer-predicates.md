Implement bigint integer predicates from the MPU reference with existing bigint factors.
Add the bigint_predicates oracle generator, replay tests, and native agreement properties.
Keep native APIs unchanged and propagate factorization failures explicitly.
Run offline focused tests and the required gate; record unavailable checks here.

Read native signatures and C factor criteria; implement practical factor predicates first. Congruent-number, Pillai, and inverse-totient searches need feasibility review.

Host is wowbagger, not the requested aragog/wonko. Build-host constraint may block execution; oracle must stay offline.

Implemented 15 predicates, including inverse-totient recursion. All integer arguments use BigUint; factor errors remain explicit. STATUS entries were already checked.

Source generation needed correction after command text truncation. Corrected the two generated declarations; next add fixtures and tests.

About five minutes remain: oracle generation is running; focused tests are launched through tools/test.sh. Two names remain native: congruent-number uses quadratic-form enumeration and Pillai scans factorial residues up to n.

Added generator and tests for all 15 ports. Offline oracle uses vendored PP because .perl-local is absent; perl-env would attempt forbidden network bootstrap.

Oracle generated 2679 rows offline from vendored MPU 0.75. Required test and gate wrappers are running on the available wowbagger host; designated-host reruns remain necessary. No network or git writes were used.
