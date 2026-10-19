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
