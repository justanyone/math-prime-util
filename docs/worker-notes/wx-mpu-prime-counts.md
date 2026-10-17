Port the count family from the MPU 0.75 C sources into src/count.
First check the existing APIs, fixtures, and reference bounds.
Keep changes within count code, tests, benchmarks, and this notes file.
Run focused offline tests and the required gate; record any incomplete algorithms.
Existing sum_primes is in sieve; count is a stub. Start with the Lehmer exact path and phi recurrence, with fixture and property tests. LMO remains a separate required port.
Implemented the Lehmer split with local caches and the phi recurrence. Added fixture coverage for counts, phi, and the existing sum_primes; next run focused tests.
Added the C tablephi wheel tables to reduce recursion. Focused unit tests are running; full fixtures include counts through 17.6 trillion, so runtime is a review target.
Unit counts and concurrent duplicate calls passed. Compatibility and property tests are running offline; next check the gate and address concrete failures.
Gate found doc_markdown and a provably bounded conversion flagged as a possible panic. Fixed both without changing the API. Compatibility fixtures are still running.
The second gate is running after lint fixes. No network, fixture edits, dependencies, root exports, or STATUS changes were needed. LMO, analytic bounds, nth-prime APIs, Chebyshev functions, count tables, and MPU benchmarks remain outstanding.
