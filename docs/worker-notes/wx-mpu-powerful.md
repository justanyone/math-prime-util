Port powerful.c and powerfree.c into the existing families module.
Keep the C algorithms and use existing integer and factoring helpers.
Add fixture compatibility, algebraic tests, and hot function benchmarks.
Run the offline focused tests and the required gate; leave tools and fixtures unchanged.
Read the C recurrences and fixture boundaries. Private factoring dependencies are unavailable; use local helpers. Wide sums must cover oracle values beyond u64.
Implemented powerful generation/count/sum recurrences and the powerfree Mobius/Mertens formulas. Next add the compatibility harness and run it.
Focused fixture tests launched offline. Added property tests for counts, sums, parts, inverse ranks, boundaries, repeated calls, and concurrent calls. Added Criterion benchmark source.
Initial fixtures passed 8/11 functions. Corrected zero-k API mapping and single-bound list arguments. Focused compatibility and property tests are running; begin the gate next.
Ten fixture functions now pass. List fixtures encode an empty list as undef; adapted the harness. Fixed sieve index lint findings. Gate rerun is underway.
Added the C nth-powerful rank limits, small-rank table, and fitted search bounds. Wide sums deliberately extend the C native overflow path. Native-only fixture skips name signed, bigint, and out-of-u32 inputs.
All 11 compatibility tests and all 3 property/boundary tests pass offline. Gate reached test/benchmark lint checks; fixed semicolon and empty-assert style findings. No external state or shared mutable cache is used.
Final gate is running after focused tests passed. Benchmark source is added without changing Cargo.toml. Tools, fixtures, crate root, and STATUS.md remain untouched as required.
The gate has passed fmt, clippy, and the offline Rust 1.85 build and is running the full test suite. Native fixture skips cover inputs/results outside the unsigned APIs; no Perl or network was required.
