I will review the existing regression harness and all 14 reported mutation sites.
I will complete deterministic killing tests and fix any demonstrated algorithm bug.
I will run focused offline tests, the gate, and a two-job mutation rerun.
The requested new test path already exists in this restarted checkout; I will finish it in place.
Reviewed existing deterministic cases; focused tests are running. Next I will target the 14 exact operator changes with cargo-mutants.
The initial filter matched no tests; corrected it to mutation_regressions. The gate is running. One listed guard was removed in the inherited change as equivalent; I will record that exception explicitly.
