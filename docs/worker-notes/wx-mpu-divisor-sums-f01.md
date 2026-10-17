I will reassess the existing divisor sums and their C reference algorithms.
I will move only the four assigned functions into separate arithmetic files.
I will preserve fixture compatibility and add boundary and algebraic tests.
I will run the focused offline tests and the required gate without git writes.
The four APIs already exist. I moved them into separate files and retained the existing PP fallback needed by the oracle; next I will verify native overflow behavior and add focused regressions.
Focused offline compatibility and property tests are running. Review found that tau computes p^11 even for exponent one, unlike PP.pm; I will test this boundary and retain the C aliquot prime-power shortcut.
Extraction initially missed the tau helper's is_square import; that is fixed. Added C prime-power aliquot handling, divisor-sum properties, concurrent repeated calls, overflow boundaries, and benchmarks for the remaining APIs.
The new tau(3011) regression failed with None before the fix. The exponent-one shortcut is now in place, and the full arithmetic compatibility and property tests are running offline.
All 53 arithmetic integration tests passed offline, including the previously failing tau regression. The gate is checking all targets; added small class-number helper cases and final wide-sum overflow checks for line coverage.
The first gate stopped on a test literal without digit separators. Fixed that clippy warning; rerunning the gate after the final coverage tests. No dependency, fixture, tooling, or status changes were needed.
Final validation phase, with about five minutes reserved for the gate. The regression, fixture tests, algebraic laws, and concurrency tests have passed; the final helper and overflow tests are compiling with the gate.
