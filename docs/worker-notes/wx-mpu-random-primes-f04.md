Implement the four assigned random functions in separate module files.
Read MPU algorithms and use the existing portable random state.
Keep tools, fixtures, crate root, and generated STATUS.md unchanged.
Add fixture compatibility, algebraic tests, and benchmarks; run offline checks.
Confirmed existing thread-local RNG and fixture seed format; locating exact native algorithms before implementation.
Implemented all four APIs. The factor module is empty, so added a private native Brent splitter based on factor.c for the two callers.
Added compatibility tests for every assigned fixture family and property tests for bounds, products, uniqueness, replay, and thread isolation. Focused tests are running offline.
All four compatibility tests and six property/boundary tests passed. Gate found only missing benchmark semicolons; corrected them before rerunning.
Gate reached green. Tightened randperm to retain MPU's fixed 16-slot map and u16 dense table; added Brent failure-path checks and a dense large-n case before final validation.
Final validation: native oracle fixtures, property tests, private factor tests, and offline full suite pass; final tools/gate.sh prints == gate: green. All four APIs, examples, and benchmarks are complete. Bigint inputs remain deferred by s <<cut 105 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 80 bytes)
