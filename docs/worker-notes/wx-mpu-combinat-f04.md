Implement the four combinat functions in separate files and preserve sibling code.
Use the MPU tables and rational algorithm, with checked native-width results.
Use existing offline oracle fixtures; do not change tools, fixtures, or STATUS.md.
Validate compatibility, algebraic properties, and the repository gate offline.
The C fraction routines have narrow native limits; inspect MPU's Seidel fallback to cover the requested i128/u128 results without new dependencies.
Implemented the C primorial tables and binary splitting; Bernoulli uses the C table then MPU's Seidel fallback with fixed three-limb intermediates.
Added oracle comparisons, algebraic properties, repeated/concurrent calls, overflow boundaries, and Criterion benchmarks. Focused offline tests are running; reduced the cross-product property domain to avoid test-only overflow.
All four oracle tests and six property/boundary/concurrency tests pass offline. Fixed the Seidel direction state to match Perl's pre-decrement loop. Running the required gate next.
Gate lint found literal separators, midpoint spelling, and trailing closure semicolons; corrected these without changing results. No dependencies or shared tooling changed.
