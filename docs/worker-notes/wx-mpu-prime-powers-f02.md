Port the four prime power counts from MPU 0.75 in separate module files.
Reuse the existing prime count and integer root APIs where they match C.
Add fixture compatibility, algebraic properties, and a small benchmark.
Keep tools, fixtures, STATUS.md, and git metadata unchanged as required by this task.
The four root-sum ports are written. Prime count bounds are absent; add private C formula dependencies in the task files. Large oracle values require more precision than f64.
Added the C approximation tolerance, both C bound formulas and float table, four compat tests, properties, and benchmarks. Focused offline tests are running; check large-input precision next.
The first focused build found that is_prime returns u8; corrected the property predicate. Investigating whether MPU's NV return rounding matters at u64::MAX.
Focused tests are executing the large exact counts. Formatting is applied; next resolve any oracle precision differences before the gate.
