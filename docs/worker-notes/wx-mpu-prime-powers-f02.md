Port the four prime power counts from MPU 0.75 in separate module files.
Reuse the existing prime count and integer root APIs where they match C.
Add fixture compatibility, algebraic properties, and a small benchmark.
Keep tools, fixtures, STATUS.md, and git metadata unchanged as required by this task.
The four root-sum ports are written. Prime count bounds are absent; add private C formula dependencies in the task files. Large oracle values require more precision than f64.
Added the C approximation tolerance, both C bound formulas and float table, four compat tests, properties, and benchmarks. Focused offline tests are running; check large-input precision next.
The first focused build found that is_prime returns u8; corrected the property predicate. Investigating whether MPU's NV return rounding matters at u64::MAX.
Focused tests are executing the large exact counts. Formatting is applied; next resolve any oracle precision differences before the gate.
The u64::MAX approximation differs by 859 with f64. Added two-component arithmetic for the existing C logarithmic integral series and large Mobius sum. The gate also found benchmark semicolons; fixed them.
All approximation fixtures now pass, including u64::MAX. Adjusted bounds to retain MPU's Li NV rounding before long-double arithmetic. Rechecking bounds, properties, and the full gate.
Both bound fixture tests and the initial properties pass. Added large-input precision coverage and concurrent repeat calls because all counts must remain stateless. The gate rerun includes the deliberate precision-cast lint allowance.
The gate passed fmt, clippy, and MSRV and is running existing suite tests. The named origin/dev ref is absent in this isolated checkout; inspect the initial snapshot to preserve any pre-existing family test content.
Recovered the pre-existing lucky-family tests and benchmarks from the initial snapshot and retained them alongside the new cases. This corrects an accidental replacement of shared family files. Revalidate the combined files.
The combined diff now preserves the original family cases. Full offline validation is still running in the existing prime-count compatibility tests; no new failure has been reported.
At 06:53 UTC, the new focused tests are green. Full validation remains in the existing count-family oracle tests, which exceed one minute in this debug build. Continue waiting within the task deadline.
Review found and fixed a fractional-high-component rounding edge in Wide::integer. Added direct regression cases for floor, nearest, and negative residuals. This requires a final gate rerun after ongoing full tests finish.
At roughly six minutes remaining, the implementation and regression tests are written. Full-suite and gate validation are still running offline; preserve the completed source and record any deadline blocker.
The rounding regression passes. Existing count oracles have run for several minutes in debug mode. Start the final gate with CARGO_PROFILE_TEST_OPT_LEVEL=2 to finish the same tests faster without changing repository tooling or test semantic <<cut 2 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed -21 bytes)
About five minutes remain. All four ports, docs, benchmarks, preserved lucky cases, and new regressions are on disk. The optimized final gate is compiling its test profile; earlier gates passed fmt, clippy, and MSRV.
- 2026-10-17T06:57:15Z launcher: no commit for 60 s; 577 s elapsed. Files changed so far: benches/families.rs, docs/worker-notes/wx-mpu-prime-powers-f02.md, src/families/mod.rs, src/families/prime_power_count.rs, src/families/prime_power_count_approx.rs, src/families/prime_power_count_lower.rs, src/families/prime_power_count_upper.rs, tests/compat_families.rs, tests/prop_families.rs.
The final focused rerun passed all 14 family compat tests, all 7 family properties, and the precision regression. The optimized gate is still in existing count-family compatibility tests; no new failure is visible.
