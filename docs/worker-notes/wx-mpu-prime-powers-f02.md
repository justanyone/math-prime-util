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
