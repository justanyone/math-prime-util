Port the count family from the MPU 0.75 C sources into src/count.
First check the existing APIs, fixtures, and reference bounds.
Keep changes within count code, tests, benchmarks, and this notes file.
Run focused offline tests and the required gate; record any incomplete algorithms.
Existing sum_primes is in sieve; count is a stub. Start with the Lehmer exact path and phi recurrence, with fixture and property tests. LMO remains a separate required port.
