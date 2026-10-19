Implement bigint set and vector operations with native-compatible behavior.
Read PLAN sections 5–8 and use MPU C and PP sources for algorithm semantics.
Add a pure-Perl fixture family and native-equivalence property tests.
Run offline focused tests and the required gate; record host and tool blockers.
Implemented all 32 names with local state, generic signed/unsigned element operations, and the native merge logic. Local Perl bootstrap is absent; use the vendored PP oracle without network.
Generated the offline pure-Perl oracle from reference/mpu/lib. All 32 functions are now written; next add complete replay and native-equivalence tests.
Replay tests cover every generated row; property tests cover all names, mutation retries, signed values and concurrent calls. Rust execution remains blocked by the host restriction.
About five minutes remain: fixed oracle calling conventions for toset and in-place sorts. Fixtures are regenerated, deterministic, and include signed and empty cases. Host restriction still prevents Rust tests and gate.
Expanded native properties to large representable integers. Setbinop now normalizes input pairs like MPU C/PP, with a callback-count regression test for duplicate inputs.
Validation: cargo fmt --all --check and git diff --check pass. The generated family has 1,582 rows across all 32 names and no unstable Perl reference addresses.
Offline oracle command: MPU_NO_XS=1 PERL5LIB=reference/mpu/lib perl tools/gen_fixtures.pl bigint_sets. The prescribed perl-env bootstrap was unavailable because local::lib is absent and its fallback uses forbidden curl/network.
Required follow-up on aragog or wonko: tools/test.sh tests/compat_bigint_sets.rs tests/prop_bigint_sets.rs, then tools/gate.sh and the factory coverage check. None ran here because hostname is wowbagger and the task explicitly restricts Rus <<cut 34 chars by kit>>
All requested STATUS.md names were already checked, so STATUS.md is unchanged. No native signature, dependency, README, generated status file, or Git metadata was changed.
Finished implementation and test authoring. Gate and 95% coverage remain unverified on this host; no implementation exclusions or known oracle defects were found.

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 10 bytes)
