Implement the rational API and test it against the existing MPU fixtures.
Keep changes in the rational module, tests, benchmark, status, and this note.
The named reference/mpu/rational.c and rational.h are absent in this checkout.
Use the available oracle cases to validate the possible implementation; record the source-port blocker.
The oracle includes signed and bigint fallback cases. Use signed continued fractions and checked native tree indices; test representable inputs and name skipped fallback cases.
Implemented continued fractions, tree traversals, and Farey operations. Farey rank currently uses a denominator sieve; source-equivalence remains unverified because the submodule is empty.
Added one oracle test per requested function, continued-fraction/tree/Farey properties, invalid-input checks, repeated and concurrent calls, and hot-function benchmarks. Focused oracle tests are running offline.
