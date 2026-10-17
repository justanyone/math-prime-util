Port the MPU 0.75 lucky-number algorithms into src/families.
Keep the existing fixtures and tooling unchanged. Add compatibility, property, and benchmark coverage.
Read PLAN sections 5–8 and the named entry points before implementation.
Validate offline with the focused test wrapper and the final gate. No git writes.
The existing families module is empty. Fixtures cover range calls, zero-index undef, large approximations, and exact values through millions. Port the C tables, presieve, rank sieve, backward sieve, and estimator coefficients.
