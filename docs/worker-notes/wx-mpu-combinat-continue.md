# wx-mpu-combinat continue notes
- Plan: finish src/combinat/mod.rs (23 functions still missing), tests/compat_combinat.rs, tests/prop_combinat.rs, lib re-exports already there.
- Existing: src/combinat/mod.rs (102 lines), compat and prop test stubs (59, 68 lines).
- Not committing: launcher commits; task text forbids git writes.
- Added binomial, catalan_number, bell_number, fubini, primorial, pn_primorial, consecutive_integer_lcm; ticked. Next: run compat tests.
- Compat tests added; retrying focused run (linker cc missing earlier).
- Last: build impossible here (cc linker missing). Ticked 7 fns, code unverified by tests. Remaining: stirling, powersum, integer_complexity, bernfrac, harmfrac, is_congruent_number, is_polygonal, is_pillai, is_happy, hclassno, numtoperm, permtonum, partitions, partitionsq, frobenius_number, prop tests, benches, C source check (reference/mpu empty).
