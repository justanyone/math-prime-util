Implement the four assigned random functions in separate module files.
Read MPU algorithms and use the existing portable random state.
Keep tools, fixtures, crate root, and generated STATUS.md unchanged.
Add fixture compatibility, algebraic tests, and benchmarks; run offline checks.
Confirmed existing thread-local RNG and fixture seed format; locating exact native algorithms before implementation.
Implemented all four APIs. The factor module is empty, so added a private native Brent splitter based on factor.c for the two callers.
