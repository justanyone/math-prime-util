Implement the four combinat functions in separate files and preserve sibling code.
Use the MPU tables and rational algorithm, with checked native-width results.
Use existing offline oracle fixtures; do not change tools, fixtures, or STATUS.md.
Validate compatibility, algebraic properties, and the repository gate offline.
The C fraction routines have narrow native limits; inspect MPU's Seidel fallback to cover the requested i128/u128 results without new dependencies.
Implemented the C primorial tables and binary splitting; Bernoulli uses the C table then MPU's Seidel fallback with fixed three-limb intermediates.
