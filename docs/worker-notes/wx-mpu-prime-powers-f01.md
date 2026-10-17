Port the four prime power functions from the native MPU algorithms.
Keep each public function in its own file and use the existing integer and prime APIs.
Use the committed families fixtures without changing tools or oracle data.
Add compatibility, algebraic, boundary, and benchmark coverage; run offline checks.
Read the native reference and existing test patterns; implemented all four functions with the C scan threshold and neighbor bounds.
Added all oracle tests, prime-root and neighbor laws, boundary and concurrent repeat-call tests, plus four benchmarks. Focused offline tests are running; corrected undef handling for previous prime powers.
Focused compatibility and property tests pass (22 tests). Three next-prime-power oracle results are 2^64; the test documents this native-API limit and checks None. Running the full offline gate next.
The gate found four missing benchmark semicolons under pedantic clippy. Fixed them and added a no-higher-powers merge-path assertion; rerunning the gate.
