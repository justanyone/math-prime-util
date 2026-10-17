Read the plan and existing Lucas implementation before editing.
Compare the failed paths with MPU 0.75 and fix the arithmetic and panic harness.
Add algebraic property tests and hot function benchmarks.
Keep tools, fixtures, crate root, and generated status unchanged as required.
Run focused offline tests and the merge gate; record any blocker.
Fixed even modulus division and removed unused exact-ladder power products; added expected-panic fixture checks.
Added property, bound, repeat-call and concurrency tests plus hot function benchmarks. Zero-modulus oracle undef is checked against the documented C panic contract.
