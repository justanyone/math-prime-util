Continuing the existing estimate mutation regression tests; no existing test files will be edited.
Check remaining mutants against public callers and MPU reference algorithms, distinguishing wrong bounds from equivalent or non-tight changes.
Add reachable regression cases in tests/mutants_estimates.rs, then run focused tests, targeted cargo-mutants, and the required gate.
Do not mark this task READY; record unresolved mutants and environment limitations explicitly.
Reviewed MPU lucky fits and public call domains: the n=7 simple_approx comparison is unreachable; harmonic tiny-term comparison appears f64-equivalent. Preparing targeted mutation baseline.
Added lucky fit boundary sandwich tests through 4e9 and independent fourth-almost-prime checks; focused tests and all 13 requested mutations are next.
Focused regression suite passes all 11 tests. cargo-mutants rejected --offline; restarted with CARGO_NET_OFFLINE=true. Required gate is running independently.
MPU inverse_interpolate.c::interpolate_with_approx confirms that interpolation is only a starting guess followed by exact count correction; comparing branch reachability before claiming equivalent mutants.
Formal branch argument: finite interpolation bounds were sampled only with count error >=20,000, so a bracket spanning both sides has width >=40,000. The span<=19,999 midpoint arm is unreachable; other guess changes require runtime/performance evidence, not a different exact-answer assertion.
Resolution detail (source references): lucky_numbers.c::_simple_lucky_count_approx is only reached from the Rust simple_lower when n>9000. Its n<7 versus n<=7 mutant cannot affect a public call; directly at 7 the mutant would incorrectly re <<cut 21 chars by kit>>
Resolution detail: PP.pm::harmreal tiny-term cutoff discards terms below 1e-44. For native n>2002, the accumulated correction starts at magnitude at least 1/(12*(u64::MAX as f64)^2), approximately 2.4e-40, with f64 spacing approximately 4e- <<cut 139 chars by kit>>
Resolution detail: inverse_interpolate.c::interpolate_with_approx recomputes exact_count at every guess and after exhausting 18 iterations. The final forward/backward scan restores the exact rank, so changed guesses can remain exact-answer  <<cut 139 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 225 bytes)
Mutation baseline passed; first three mutants survive (harmonic cutoff, unreachable lucky n=7, and lucky division-to-multiplication seed). Added geometric public-estimate ordering checks across native scales to probe bad seed brackets.
The new scale sweep is running; the first mutation pass remains on its original copied test file, so a final pass must include the completed test file.
All five lucky mutants survived the first targeted run. This confirms current public-result tests do not distinguish their search seeds; it does not by itself prove all mutated bounds valid. Retain explicit unresolved status rather than claiming kills.
Added exact-rank checks at indices 1e6 and 1e7 for k=3 and k=4: small independent-sieve cases can stop within the 20,000 tolerance without entering the correction branches.
Gate has passed formatting and clippy in all four feature configurations plus the MSRV build; all-features full tests are compiling/running.
First complete run: exactly 13 requested mutants, 3 Ramanujan mutants caught and 10 missed. Fixed test-only u32/u64 call mismatch and overflow in the geometric input generator; these are not production defects.
Final focused suite rerun and second 13-mutant run launched with completed tests. About five minutes remain in the worker budget; priority is completing validation and leaving precise unresolved findings.
Per-mutant disposition from first pass (all source line numbers unchanged):
- real:763 (< to ==): missed; added harmonic direct-sum checks pass. Precision-equivalence argument needs a complete rounding proof before formal closure.
- lucky:327 (< to <=): formally unreachable through public callers (simple_lower only calls simple_approx above 9000). The changed direct result at 7 is not a public-bound defect.
- lucky:337 (/ to *): missed; negative fit saturates to zero, weakening the lower search endpoint. No wrong public bound observed at tested boundaries.
- lucky:338 (<= to >): missed; changes fitted search endpoint. Public bound validity across the full affected range remains unproved.
- lucky:339 (/ to *): missed; negative fit saturates to zero, weakening the lower search endpoint. No wrong public bound observed at tested boundaries.
- lucky:340 (<= to >): missed; changes fitted search endpoint. Public bound validity across the full affected range remains unproved.
- ramanujan-upper:121 (> to >=): caught at n=3,999,654,659. Mutant returns 205,925,882,256 versus C-compatible 205,928,435,147. Whether the smaller mutant remains an upper bound for the actual Ramanujan prime is not established by equality  <<cut 8 chars by kit>>
- ramanujan-upper:124 (delete minus): caught at n=512,000,001. Mutant returns 24,178,264,828 versus 24,166,874,695: a looser upper bound, not a demonstrated bound violation.
- ramanujan-upper:146 (< to <=): caught at n=7140, Axler index 15360. Mutant returns 168418 versus 168432. Equality test establishes MPU formula mismatch, not failure to bound the exact Ramanujan prime.
- nth-almost:155 (>= to <): midpoint arm is unreachable with span<=19,999. Finite lower/upper bracket counts differ by at least 40,000; one unset endpoint also cannot create such a narrow bracket for a reached k=3/4 interpolation call. Muta <<cut 108 chars by kit>>
- nth-almost:157 (<= to >): missed; exchanges interpolation fallback directions. Exact correction preserves rank for terminating executions; worst-case runtime remains unverified.
- nth-almost:160 (- to +): missed; can leave the bracket and overflow. Needs a reaching input or proof of unreachability; not formally resolved.
- nth-almost:160 (- to /): missed; can leave the bracket and lengthen final exact scan. Needs a reaching input or proof of unreachability; not formally resolved.

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 68 bytes)
Leaving source algorithms unchanged: no existing production bound violation has been demonstrated. Notes distinguish MPU result mismatches, provably unreachable call domains, and unresolved validity/performance questions.
Second run again reports the same ten survivors so far, including new large-index and geometric tests. Launched the single explicit whole-suite tools/test.sh run; required final gate continues.
Final targeted confirmation complete: 13 tested in 70 seconds, 3 caught, 10 missed. Acceptance remains PARTIAL; no READY marker or production algorithm change is justified by this evidence.
About five minutes remain: final focused suite is green (13/13); targeted mutation run confirms 3 catches and 10 misses. Full gate and explicit whole suite are waiting on the expensive prime_count oracle case; no completion claim will be made before their results.
Validation still running in prime_count_oracle; no new implementation changes. Final report will retain the ten missed mutants and the required unresolved full-domain bound checks.
- 2026-10-21T01:30:18Z launcher: no commit for 60 s; 485 s elapsed. Files changed so far: docs/worker-notes/pc1-mpu-close-mutation-survivors-in-lucky-ramanujan-upper-t2021.md, tests/mutants_estimates.rs.
Minute update: final gate is still running the prime-count oracle test; earlier gate has progressed into bigint property tests. No gate failure has been observed, but green is not yet printed.
Stronger formal resolution of lucky:337 and lucky:339: their reached input intervals are respectively 1,000,000<n<=10,000,000 and 10,000,000<n<=100,000,000. Replacing logn/305 or logn/304 with multiplication makes the factor negative throug <<cut 289 chars by kit>>
Stronger formal resolution of real:763: the first correction has magnitude a=1/(12*n^2), and is never below 1e-44 for any native n. For a later term with denominator d and power n^(2j+2), j>=1, being below 1e-44 implies its ratio to a is at <<cut 501 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 740 bytes)
Minute update: full validation has reached bigint property tests and continues; no green marker yet. Formal notes now establish unreachable lucky n=7, valid zero lower endpoints for both multiplication mutants, and f64 equivalence of the harmonic cutoff mutant.
Required final gate completed successfully and printed == gate: green (fmt, all clippy feature sets, MSRV, all-features tests and docs). The explicit tools/test.sh whole-suite invocation is still finishing; task remains PARTIAL because mutation acceptance is unmet.
Boundary bound assessment for ramanujan-upper:146: independent Eratosthenes sieve gives min(pi(x)-pi(floor(x/2)))=7158 for every integer x in [168418,168432], against n=7140. Combined with the existing valid upper bound 168432 for all larger x, mutant upper 168418 still bounds R_7140. This mutant is non-tight/formula-incompatible at the used boundary, not a bound violation.
Final validation: tools/test.sh tests/mutants_estimates.rs passed all 13 tests; tools/test.sh whole suite completed successfully, including 837 doctests; tools/gate.sh printed == gate: green. Final targeted cargo-mutants run across the four <<cut 118 chars by kit>>
Final status: PARTIAL, not READY. Four misses have formal equivalence/valid-lower-endpoint explanations above; six misses still require stronger proof or killing inputs, and the smaller Axler boundary bound at n=3,999,654,659 still needs va <<cut 258 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 326 bytes)
