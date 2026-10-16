Implement the rational API and test it against the existing MPU fixtures.
Keep changes in the rational module, tests, benchmark, status, and this note.
The named reference/mpu/rational.c and rational.h are absent in this checkout.
Use the available oracle cases to validate the possible implementation; record the source-port blocker.
The oracle includes signed and bigint fallback cases. Use signed continued fractions and checked native tree indices; test representable inputs and name skipped fallback cases.
Implemented continued fractions, tree traversals, and Farey operations. Farey rank currently uses a denominator sieve; source-equivalence remains unverified because the submodule is empty.
Added one oracle test per requested function, continued-fraction/tree/Farey properties, invalid-input checks, repeated and concurrent calls, and hot-function benchmarks. Focused oracle tests are running offline.
Focused tests and the gate cannot link because this image has no cc/gcc/clang. Dependencies are cached; force Cargo offline. Try the installed ld with explicit startup objects, without adding repository tooling.
Library clippy found two manual ilog2 expressions; replaced them with ilog2. Testing ld with a build-directory libgcc_s linker-name symlink; no tracked build tooling was added.
All 14 requested APIs have implementations, examples, and status ticks. Library clippy passes. Source fidelity, the C native bounds, and Farey performance still need review against the missing reference before this can count as a faithful p <<cut 4 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed -19 bytes)
The direct-ld attempt reached dependency compilation but startup objects broke proc-macro loading. A temporary driver under ignored target/ now adds startup objects only for executables; rerunning the two focused suites and full gate offlin <<cut 2 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed -21 bytes)
Fixed MPU's negative continued-fraction form and Farey insertion-rank endpoint behavior. Fixed the property borrow and benchmark lint findings; focused suites and the gate were rerun.
All 14 oracle tests and four property tests pass. The gate passed fmt, all-target clippy, MSRV, and ordinary tests; doctests still used cc, so pass the temporary linker settings to rustdoc as well. Roughly eight minutes remain; missing C so <<cut 45 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 21 bytes)
