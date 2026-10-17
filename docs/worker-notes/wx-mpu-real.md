Port the real-number functions from the read-only MPU reference to src/real/.
Keep the existing oracle fixtures and tools unchanged, as the revised task requires.
Add compatibility, algebraic, validation, and concurrent-call tests plus benchmarks.
Run the focused offline tests and the repository gate. The launcher owns commits.
Located the native C paths and the Perl-only Bernoulli/harmonic fallbacks; porting the tables and fixed iteration bounds without dependencies.
Implemented the C tables, integral series, zeta, Riemann R, Lambert W, inverse li, and pi spigot. Next add the remaining fallbacks and oracle tests.
Nine oracle functions now pass. Native inverse-li must use the requested relative f64 tolerance at large inputs; Perl bigint-only inverse cases are outside the native u64 API. Fixed Seidel traversal and NaN comparisons.
All ten compatibility tests and all seven property/validation/concurrency tests pass offline. The gate found a style issue in the translated Lambert approximation; converting its branches to Rust expressions.
Finished the pedantic lint fixes in the port and benchmark. The full offline gate is now running; fixture generation is intentionally unchanged because the real family is already committed and the task forbids fixture/tool edits.
