Port the four twin and semiprime functions from the MPU C reference.
Keep each function in a separate families source file and reuse crate primitives.
Add fixture compatibility tests, algebraic tests, and criterion benchmarks.
Leave tools, fixtures, STATUS.md, and git metadata unchanged per task rules.
Run focused offline tests and the required gate; record any blockers here.
Reference methods and fixture input shapes reviewed; implement isolated source files and tests next.
Implemented semiprime marking, exact prime-count sum, and reference approximation tables. High narrow ranges resolve known prime factors early to avoid a full four-billion bound walk.
All four functions, fixture tests, property tests, concurrency/repeat-call checks, and benchmarks are written. Focused offline tests are running.
Validation is waiting on the Cargo build-directory lock; no fixture generator or network was used.
The first gate reached clippy. Fixed reference-table literal lint allowances and documented the range-size panic; waiting for focused results.
Focused offline validation passed: all 4 compatibility tests and all 4 property/regression tests. The gate is rerunning after clippy fixes.
