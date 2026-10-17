Port the real-number functions from the read-only MPU reference to src/real/.
Keep the existing oracle fixtures and tools unchanged, as the revised task requires.
Add compatibility, algebraic, validation, and concurrent-call tests plus benchmarks.
Run the focused offline tests and the repository gate. The launcher owns commits.
Located the native C paths and the Perl-only Bernoulli/harmonic fallbacks; porting the tables and fixed iteration bounds without dependencies.
Implemented the C tables, integral series, zeta, Riemann R, Lambert W, inverse li, and pi spigot. Next add the remaining fallbacks and oracle tests.
