Port fubini, factorial, multifactorial, and subfactorial from MPU util.c.
Keep each function in its own file and preserve the C bounds and methods.
Use the existing combinat fixtures without editing tools or fixture data.
Add compatibility, algebraic, boundary tests, and criterion benchmarks.
Run offline focused tests and the required gate; leave git metadata unchanged.
The fixture includes exact Fubini values beyond util.c's conservative n < 16 bound; inspect the reference fallback before choosing the u64 boundary.
Implemented the four functions. Fubini preserves the C sum below 16 and uses MPU's PP recurrence for 16–18; multifactorial rejects zero steps like XS.
Added oracle tests with explicit signed/bigint input skips, algebraic properties, overflow edges, zero-step validation, repeated concurrent calls, and benchmarks. Starting focused offline tests.
Focused tests are compiling offline. The implementation has no global cache or shared mutable state; concurrent repeated boundary calls check independence.
Focused offline tests passed: four oracle tests and seven property/boundary/concurrency tests. Running the required gate next.
The gate found four clippy semicolon formatting errors in test/benchmark closures. Added the required semicolons and rerun the gate.
