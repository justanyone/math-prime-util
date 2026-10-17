Port the MPU 0.75 lucky-number algorithms into src/families.
Keep the existing fixtures and tooling unchanged. Add compatibility, property, and benchmark coverage.
Read PLAN sections 5–8 and the named entry points before implementation.
Validate offline with the focused test wrapper and the final gate. No git writes.
The existing families module is empty. Fixtures cover range calls, zero-index undef, large approximations, and exact values through millions. Port the C tables, presieve, rank sieve, backward sieve, and estimator coefficients.
Implemented the C tables, block-copy small sieve, compressed bit sieve with a rank tree, forward/backward rank algorithms, and estimator formulas. Added all ten fixture tests, rank/membership properties, concurrent repeat-call checks, and b <<cut 46 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 22 bytes)
