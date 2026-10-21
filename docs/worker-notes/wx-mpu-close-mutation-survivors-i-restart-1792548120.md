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
Confirmed the 14th reported mutant is equivalent (s > 1 versus s >= 1 before a primality predicate that rejects 1). Started the required single tools/test.sh full-suite run. Mutation baseline is still running.
The mutation runner is building integration targets for its baseline despite the focused test argument. I am allowing the offline build to finish; the main full-suite build is also active.
Mutation baseline failed before testing: disk quota exceeded while compiling integration targets in the copied target directory. Retrying once with no target copy, bigint only, --lib for build and test, and debug/incremental output disabled.
The main gate reached the bigint certificate compatibility tests. The smaller mutation retry has started; no source or regression assertion has failed.
The reduced mutation build is still running. The gate continues through compatibility tests without a failure. The separate tools/test.sh full run remains active.
Reduced mutation baseline passed: 28 s build, all 14 focused tests pass. The 13 selected mutants are now running with nice 15 and two jobs. No network was used.
The reduced offline cargo-mutants rerun completed successfully. I will record its exact kill count and wait for the required gate and full-suite results.
Mutation rerun result: 13 mutants tested in 2m; 13 caught; no survivors or timeouts.
Rerun used CARGO_NET_OFFLINE=true, nice -n 15, cargo mutants --features bigint -j 2 --copy-target false -C=--lib, both requested files, and an exact operator/location regex.
The tests cover check_ecpp + to - and + to *, both bls75 remainder changes, curve_orders * to +, and PolyMod::mul || to &&.
They also cover urandomm subtraction and remainder, final-bin subtraction, factored acceptance && and <, MR base addition, and the semiprime <= comparison.
The original random_factored_integer s > 1 to s >= 1 mutant is equivalent: s=1 fails is_prob_prime. The inherited source removes this redundant guard; no test can distinguish it.
No correctness bug was found. No dependency, fixture, build tooling, credential, or service was changed.
tools/gate.sh finished successfully. Checking for its required green marker and the separate tools/test.sh result before finishing.
Verified == gate: green: fmt, all clippy feature sets, MSRV, full tests, and docs passed offline. Only the separate tools/test.sh whole-suite invocation is still active.
About five minutes remain; all required runs have now finished. No further source changes or test reruns are needed.
Final verification: 14 focused tests pass; tools/test.sh full suite passes; tools/gate.sh prints == gate: green; offline cargo-mutants catches all 13 behavioral mutants (nice 15, jobs 2).
Stopped with the requested regression file complete. The equivalent guard is removed, no correctness bug was found, and no commit or push was attempted.
