I will review the existing regression harness and all 14 reported mutation sites.
I will complete deterministic killing tests and fix any demonstrated algorithm bug.
I will run focused offline tests, the gate, and a two-job mutation rerun.
The requested new test path already exists in this restarted checkout; I will finish it in place.
Reviewed existing deterministic cases; focused tests are running. Next I will target the 14 exact operator changes with cargo-mutants.
The initial filter matched no tests; corrected it to mutation_regressions. The gate is running. One listed guard was removed in the inherited change as equivalent; I will verify its exact condition from history.
All 13 inherited regressions pass. Added a deterministic unit-limit case for empty factorization. cargo-mutants rejects --offline; using CARGO_NET_OFFLINE=true instead.
Started offline cargo-mutants with nice 15, jobs 2, the two requested source files, and 13 exact operator changes. Added the unit-limit test before the rerun. Gate compilation is still in progress.
Source review matches the reference rejection, bin-width, Hasse, and strict-acceptance formulas. No correctness bug has been demonstrated. The new test also checks one acceptance draw and no prime factors for limit one.
Focused suite now passes all 14 tests. The gate has passed formatting, all four clippy feature sets, and the MSRV build; it is running the full test suite.
History confirms the removed guard was s > 1 before is_prob_prime(s), not a product guard. The >= mutant is equivalent because is_prob_prime(1) is zero. There is no reachable killing input for that mutant.
