Implement bigint set and vector operations with native-compatible behavior.
Read PLAN sections 5–8 and use MPU C and PP sources for algorithm semantics.
Add a pure-Perl fixture family and native-equivalence property tests.
Run offline focused tests and the required gate; record host and tool blockers.
Implemented all 32 names with local state, generic signed/unsigned element operations, and the native merge logic. Local Perl bootstrap is absent; use the vendored PP oracle without network.
Generated the offline pure-Perl oracle from reference/mpu/lib. All 32 functions are now written; next add complete replay and native-equivalence tests.
