Implement the additive BigUint rational API from MPU rational.c and PP.pm.
Preserve native signatures and generate the bigint_rational oracle family offline.
Test native agreement, large inputs, and validation; run the repository gate.
Farey enumeration and rank may remain native if the reference big-input work is impractical.
Added eleven BigUint functions; reused bigint gcd and modular inverse for Farey successor.
Host reports wowbagger, not the requested aragog or wonko; only offline local validation is available here.
Farey enumeration has quadratic output size; farey_rank at order above 2^64 needs impractical reference sieve/Mertens work. Keep both native; launcher commit message must state these exceptions.
Generated bigint_rational fixtures offline using vendored MPU PP: local::lib is missing, so sourcing perl-env would attempt forbidden network bootstrap.
Added all-row oracle replay and u64 native-agreement properties plus validation and repeat-call tests. Focused tests are running.
Focused and gate runs are waiting for the offline all-features build. No algorithm or fixture failures have appeared yet.
STATUS.md already checks all eleven native names; no checklist changes are needed or allowed beyond ticks.
Added address-space rejection and concurrent independent tree round trips. Shared build locking still prevents validation output; no network or bootstrap was used.
Oracle replay passed all 44 rows. The address-space test hit usize::MAX after the final CF decrement and attempted allocation; corrected the test to exceed usize after that decrement. Gate found one doc-markdown lint, now fixed.
Focused compat and property suites now pass (44 oracle rows and three tests). Bigint feature lint found needless pass-by-value in the fixture formatter; changed it to borrow the pair.
Default, bigint, and GMP clippy stages pass. The all-features gate and the one permitted full-suite test run are in progress. cargo-llvm-cov is not installed, so checking existing LLVM tools before claiming coverage.
All four clippy feature sets and MSRV build passed; gate is running tests. No LLVM coverage binaries are on PATH, so the 95% line threshold cannot be measured locally. Required aragog/wonko validation also remains external.
Checked Rust toolchain bin directories too: LLVM coverage tools are absent. Focused tests pass; all-feature full suites are still running. No public native signatures or dependencies changed.
- 2026-10-19T16:09:00Z launcher: no commit for 60 s; 393 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-bigint2-rationals.md, src/bigint/mod.rs, src/bigint/rational.rs, tests/compat_bigint_rational.rs, tests/fixtures/bigint_rational.tsv, tests/prop_bigint_rational.rs, tools/fixtures/bigint_rational.pl.
Full-suite gate remains in existing count oracle tests; no reported failures. Waiting for completion, with the focused new-family tests already green.
- 2026-10-19T16:10:32Z launcher: no commit for 60 s; 485 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-bigint2-rationals.md, src/bigint/mod.rs, src/bigint/rational.rs, tests/compat_bigint_rational.rs, tests/fixtures/bigint_rational.tsv, tests/prop_bigint_rational.rs, tools/fixtures/bigint_rational.pl.
Existing count fixtures completed; gate now runs family fixtures with no failures. About five minutes remain in the task budget; implementation and focused validation are complete, and full runs continue.
