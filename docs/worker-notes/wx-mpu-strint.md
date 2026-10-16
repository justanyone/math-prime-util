Implement the digits and checked integer helpers in src/digits/.
Use the existing digits oracle without changing fixtures or tools.
Check native C algorithms; this checkout has strops.c instead of strint.c.
Add compatibility, algebraic, boundary tests and hot function benchmarks.
Run the focused offline tests and the repository gate.
Confirmed floor, ceiling, truncating and Euclidean division differ. Shift counts may be negative. Implementing checked i128 arithmetic and native digit conversion now.
- 2026-10-16T23:09:55Z launcher: no commit for 60 s; 151 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-strint.md.
Wrote the public helpers and exact decimal parser. Generating compatibility tests next; fixtures contain extra powint/logint cases owned by the int module.
Focused compatibility run started offline. Added algebraic, overflow, validation and shift tests plus criterion benchmarks. Helpers have no shared state or launch paths.
About five minutes remain. Compatibility passes 33 of 34 tests; checking the fromdigits default-base mismatch, then running properties and the full gate. Fixed fused arithmetic cancellation overflow.
All 34 digit oracle tests and six property/boundary tests pass offline. Added four predicate oracle tests using the existing combinat fixture. First gate found comparison-chain lint; corrected it.
Gate clippy found missing semicolons in benchmark closures. Fixed those; compatibility and boundary tests remain green. Running the gate again.
Finished: all 38 functions, 38 compatibility tests, seven property/boundary tests and benchmarks are written. Offline tools/gate.sh printed == gate: green, including MSRV, clippy, tests and all 38 new examples. Fixed width fixture exclusion <<cut 96 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 72 bytes)
