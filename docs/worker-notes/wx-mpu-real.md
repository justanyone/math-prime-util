Port the real-number functions from the read-only MPU reference to src/real/.
Keep the existing oracle fixtures and tools unchanged, as the revised task requires.
Add compatibility, algebraic, validation, and concurrent-call tests plus benchmarks.
Run the focused offline tests and the repository gate. The launcher owns commits.
Located the native C paths and the Perl-only Bernoulli/harmonic fallbacks; porting the tables and fixed iteration bounds without dependencies.
Implemented the C tables, integral series, zeta, Riemann R, Lambert W, inverse li, and pi spigot. Next add the remaining fallbacks and oracle tests.
Nine oracle functions now pass. Native inverse-li must use the requested relative f64 tolerance at large inputs; Perl bigint-only inverse cases are outside the native u64 API. Fixed Seidel traversal and NaN comparisons.
