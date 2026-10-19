Implement bigint integer predicates from the MPU reference with existing bigint factors.
Add the bigint_predicates oracle generator, replay tests, and native agreement properties.
Keep native APIs unchanged and propagate factorization failures explicitly.
Run offline focused tests and the required gate; record unavailable checks here.

Read native signatures and C factor criteria; implement practical factor predicates first. Congruent-number, Pillai, and inverse-totient searches need feasibility review.

Host is wowbagger, not the requested aragog/wonko. Build-host constraint may block execution; oracle must stay offline.

Implemented 15 predicates, including inverse-totient recursion. All integer arguments use BigUint; factor errors remain explicit. STATUS entries were already checked.

Source generation needed correction after command text truncation. Corrected the two generated declarations; next add fixtures and tests.

About five minutes remain: oracle generation is running; focused tests are launched through tools/test.sh. Two names remain native: congruent-number uses quadratic-form enumeration and Pillai scans factorial residues up to n.

Added generator and tests for all 15 ports. Offline oracle uses vendored PP because .perl-local is absent; perl-env would attempt forbidden network bootstrap.

Oracle generated 2679 rows offline from vendored MPU 0.75. Required test and gate wrappers are running on the available wowbagger host; designated-host reruns remain necessary. No network or git writes were used.

Focused tests passed: 2679 oracle rows, 32 native-agreement property cases, and large boundary cases. No PP discrepancies were found. Exclusions for launcher commit message: is_congruent_number has general quadratic-form enumeration beyond practical bigint sizes; is_pillai scans factorial residues up to n, which is impractical above 2^64.

Coverage tooling is unavailable: cargo llvm-cov is not installed, and llvm-tools is absent. Followed the C totient base case rather than the extra PP bound guard. Final coverage measurement remains for factory CI.

Review: predicates hold no persistent state, locks, paths, or launch state; repeated and concurrent calls share only immutable inputs. Native signatures and dependencies are unchanged.

All four clippy feature sets passed with -D warnings. Gate is building MSRV 1.85 offline; focused tests have already passed. No measured coverage is claimed.

MSRV 1.85 passed. Started the one final whole-suite tools/test.sh run; gate is now compiling its full test suite. Final two-name exclusions are documented for the launcher commit.

Final review found no oracle disagreements and no input-validation panics. The generator is deterministic, and normal Rust tests replay committed fixtures without invoking Perl.

Full-suite compilation is still running. The code and focused fixtures are complete; waiting for the prescribed gate and final suite results.

The full gate suite has started executing tests after compilation. No failing tests have appeared so far; coverage remains unmeasured because the offline host lacks the tools.
- 2026-10-19T16:02:01Z launcher: no commit for 60 s; 485 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-bigint2-integer-predicates.md, src/bigint/mod.rs, src/bigint/predicates.rs, tests/compat_bigint_predicates.rs, tests/fixtures/bigint_predicates.tsv, tests/prop_bigint_predicates.rs, tools/fixtures/bigint_predicates.pl.

The gate and final suite are still running; no failures have appeared. Focused tests, all clippy feature sets, and MSRV have passed. Coverage and designated-host verification remain external checks.

About five minutes remain. Implementation and focused tests are complete; full-suite runs are waiting on existing long-running oracle tests. No further code changes are planned.

Added a focused runnable-doc check while the full suites execute existing oracle cases. The final diff remains limited to the bigint module, its fixture family/tests, and these required worker notes.

All 15 new runnable documentation examples passed. Full suites continue through existing tests; no new predicate failure has appeared.
