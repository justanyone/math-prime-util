Port the four assigned combinat functions from the MPU 0.75 C reference.
Keep each function in its own file and add only module declarations and exports.
Use the existing oracle fixtures; do not change tools, fixtures, or STATUS.md.
Add compatibility, property, boundary, and benchmark coverage, then run offline checks.
Read PLAN sections 5–8 and C routines; checking XS argument rules and signed H(0) before implementation.
Implemented the four reference algorithms, including the decimal happy-height table and factorial-overflow permutation prefix.
Added oracle checks, algebraic properties, invalid-input and concurrent repeat tests, and Criterion benchmarks. Running focused offline tests next.
First focused compile found mulmod returns Option; using the direct u128 C modular product path instead. Corrected a boundary-test height before rerunning.
Focused tests and the gate are compiling offline; gate currently waits on the shared Cargo build lock. No external files or fixtures changed.
All 11 focused tests pass. Clippy found two unnecessary conversion expects; replacing them with bounded native casts, then rerunning gate.
Focused oracle and property checks pass; restored C small-word product and digit-power fast paths. Benchmark formatting now follows the existing Criterion pattern.
Clippy and Rust 1.85 offline build now pass. The gate is running the complete test and documentation checks.
Full-suite unit, compatibility, and property checks have passed so far; waiting for runnable documentation examples and the final gate marker.
Finished: all four functions, runnable docs, oracle/property/boundary/concurrency tests, and benchmarks are written. Focused tests, tools/test.sh, and tools/gate.sh passed offline; gate printed == gate: green. Signed and oversized fixture arguments are documented skips. No commits or pushes performed.
