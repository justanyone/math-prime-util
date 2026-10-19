I will replace the all-primes allocation in squarefree segments with the segmented prime iterator.
For short ranges near u64::MAX, I will use the existing squarefree factor test per value.
I will add edge-focused property coverage and update only completed checklist items.
I will run the focused offline test and gate if time permits.
The edge property passes. I added a bounded segmented prime sieve because `primes_in` walks primes one by one; next I am checking the fixture test and existing squarefree property.
- 2026-10-19T15:43:55Z launcher: no commit for 60 s; 152 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-forsquarefreeint-short-range-near-u6.md, src/iter/forsquarefreeint.rs, tests/prop_iter.rs.
I noticed the edge property covered the iterator but not the callback API; I added the callback comparison too. The gate is still running against the earlier test compile, so I will rerun the focused property after it finishes.
