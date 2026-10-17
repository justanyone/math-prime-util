Port MPU 0.75 ECM and its Montgomery curve helpers to src/factor.
Read PLAN sections 5–8 and the prescribed test and gate commands.
Keep the C bounds and stages, use existing arithmetic, and leave fixtures and tooling unchanged.
Add compatibility, algebraic, and helper tests, then run the offline gate.
Inspection: reference uses tinyecm64_factor with fixed sigmas; checking the public wrapper defaults before porting.
Confirmed XS defaults: B1=4000, B2=20*B1, 100 curves; wrapper removes factors 2, 3, 5 and returns an unsplit cofactor on failure.
Implemented curve arithmetic, batch normalization, both stages, and the public wrapper; reused the existing portable Montgomery context.
Added oracle, product-law, repeated/concurrent-call, validation, and curve-helper tests plus the ECM benchmark. Focused offline tests started.
Corrected primality status comparisons. Focused tests and the offline gate are running; no fixtures or tooling were changed.
All 57 oracle cases ran. One has a different valid composite-factor partition: reference port gives 49*1856137793, fixture gives 7*12992964551. Test records both partitions explicitly; strict string parity remains a known difference.
Gate passes ECM lint checks but stops on existing clippy::float_cmp errors at src/count/mod.rs:240–242, outside task scope. Running helper tests and the full offline suite next.
Added a zero-curve early return so even maximum bounds do not allocate prime tables; added its regression test. Strict oracle partition equality and the unrelated gate warnings remain open.
