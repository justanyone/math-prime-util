Port the four assigned perfect-power functions from MPU 0.75.
Keep each public function in its own families file and use existing integer helpers.
Use committed oracle fixtures, add algebraic tests and benchmarks, and run offline checks.
Leave STATUS.md, fixtures, tooling, and the crate root unchanged as required by the task.
Implemented the exact Möbius count and the C nth approximation, interpolation, and stepping helpers; next add oracle and algebraic tests.
Added four oracle tests, full-width count and inverse laws, small exhaustive checks, repeat/concurrency checks, and hot-function benchmarks. Focused offline tests are running.
Oracle exposed undef at index zero; changed nth to Option. The single bigint result at index 4,297,615,582 is explicitly skipped; native C saturation is covered separately.
Native unit checks pass, including largest representable powers. A combined command accidentally filtered integration tests; rerunning them without the library filter. The gate is running offline.
All four oracle tests and four property/exhaustive/concurrency tests pass. Clippy found Criterion-generated missing docs; applied the existing benchmark convention and rerun the gate.
Final offline gate and the requested standalone whole-suite command are running. Diff whitespace checks pass; no dependencies, fixture files, tooling, or read-only files were changed.
Gate formatting, pedantic clippy, and MSRV build passed. Full-suite execution is currently in existing compat_count oracle tests; the assigned perfect-power tests already pass.
