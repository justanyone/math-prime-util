Port the four residue-ring functions into separate files under src/arith.
Check existing definitions against MPU util.c and preserve sibling code.
Add fixture and algebraic coverage for boundary and repeated inputs.
Run focused offline tests and the required gate; leave git metadata unchanged.
- 2026-10-17T18:41:30Z launcher: no commit for 61 s; 91 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-divisor-sums-f03.md.
Existing implementations and fixtures are present. Split the four functions into separate modules; preserve sibling functions and generated STATUS.md. Align C bounds and early checks next.
Ported the signed-bound CRT fallback, bounded order reduction, and primitive-root small-factor checks. Added scalar-oracle CRT compatibility, exhaustive group checks, shared-modulus properties, overflow regressions, and concurrent repeated  <<cut 30 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 6 bytes)
Focused tests and the offline gate are running. All task functions have C citations and runnable examples; no tools, fixtures, dependencies, crate root, or generated status changes were made.
Matched MPU's Shell-sort gap table for CRT ordering. Focused tests still hold the build lock; the gate waits for the same build directory. Diff whitespace check passed.
First checks found an unused parent import and a duplicate chinese2 compatibility test; removed both. The existing chinese2 test already reads its own oracle rows. Re-running focused tests, then the gate.
