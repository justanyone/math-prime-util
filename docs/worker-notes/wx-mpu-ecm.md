Port MPU 0.75 ECM and its Montgomery curve helpers to src/factor.
Read PLAN sections 5–8 and the prescribed test and gate commands.
Keep the C bounds and stages, use existing arithmetic, and leave fixtures and tooling unchanged.
Add compatibility, algebraic, and helper tests, then run the offline gate.
Inspection: reference uses tinyecm64_factor with fixed sigmas; checking the public wrapper defaults before porting.
