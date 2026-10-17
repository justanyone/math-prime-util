Reassess the existing powerful implementation against MPU 0.75.
Split the four requested exports into individual files and preserve sibling APIs.
Add focused oracle and algebraic regression coverage without changing fixtures or tools.
Run focused offline tests and the repository gate; record any blockers here.
The four exports now have separate files. Restored C quick rejection and count fast paths; retained shared sibling helpers.
Added independent factor-exponent, prefix, range-threshold, native-edge, and parallel repeat-call tests; added membership benchmark.
Focused tests are running offline. Oracle inspection confirms k=0 uses the public k=1 sum convention; unsigned APIs skip only signed and oversized arguments.
