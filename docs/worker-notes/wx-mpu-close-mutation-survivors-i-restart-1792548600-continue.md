Continue the estimate mutation tests from the prior run.
Fix the test macro compilation blocker in the named files.
Check all 13 survivors and distinguish changed estimates from broken bounds.
Run focused tests, cargo-mutants, and the repository gate offline.
Test macros now exist; focused helper checks are compiling. Checking exact branch values and interpolation traces before the mutation run.
Focused tests pass (33). Started the exact 13-mutant selection and offline gate. Harmonic cutoff and prime index 306 appear output-equivalent; record rather than fabricate kills.
The C reference confirms all selected branch constants. Index 306 is 2017 on both exact and bound paths. Lucky helper tests check numerical fidelity; they do not assume estimates themselves are bounds.
All 13 requested mutations were selected. Default and bigint clippy passes; GMP and mutation baseline builds remain in progress. No production bound bug has been established.
Boundary assessment (original survivor locations):
real:763 < to ==: cutoff is below 1e-44. Later terms cannot change the accumulated f64 correction or result. Equivalent output, not a bound failure.
lucky:327 < to <=: at 7 the private estimate changes 3 to 2. The public lower helper uses its separate small-input formula. No public bound failure there.
lucky:337 / to *: at 1000001 the estimate becomes zero after float-to-u64 saturation. A lower bound of zero remains valid but loses tightness.
lucky:338 <= to >: selects the wrong fit on either side of 10000000. Branch-value tests enforce the C fit; these estimates alone are not certified bounds.
lucky:339 / to *: at 10000001 the estimate becomes zero. The resulting lower bound remains valid but loses tightness.
lucky:340 <= to >: selects the wrong fit at 100000000. Branch-value tests enforce the C fit; a numerical difference alone does not prove a bound violation.
ramanujan:121 > to >=: changes D at 3999654659 from -2.0884 to -2.235 and tightens the upper estimate. Exact R_n is not available at this scale; validity of this tighter mutant bound is not established by the test.
ramanujan:124 delete minus: D changes from -2.0884 to +2.0884. It increases the Axler index and loosens the upper bound, rather than invalidating it.
ramanujan:146 < to <=: both branches return 2017 at 306, so this is an equivalent-output mutant. The upper bound holds with equality.
almost:155 >= to <: reverses the adjustment direction and can underflow. A synthetic monotone count exercises both adjustment directions.
almost:157 <= to >: changes bracket repair. The trace test verifies the C interpolation sequence rather than assuming final inversion must differ.
almost:160 minus to plus: changes bracket span. The narrow synthetic plateau exercises midpoint repair; real almost-prime counts need not reach this branch with tolerance 20000.
almost:160 minus to divide: changes bracket span and midpoint repair. The same trace exposes it.
No production arithmetic change was made: no actual bound defect was demonstrated. Two equivalent-output survivors prevent the literal 13-kill criterion.
