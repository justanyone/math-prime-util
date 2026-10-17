Reassess the existing four totient functions against MPU 0.75.
Move only this slice to separate files and retain the public API where valid.
Add focused regression and algebraic tests, then run the offline gate.
Keep fixtures, tools, crate root, and STATUS.md unchanged as required.
Existing implementations and oracle tests cover all four names. Extracted this slice into four files; next restore C direct summation and inverse prime shortcuts.
Restored the small-input direct summation path and prime-half inverse shortcut. Added boundary, repeated/concurrent-call, inverse sorting, and algebraic checks; existing oracle fixtures are unchanged.
Focused offline tests and the gate are running. The fixture contract uses native Option<u64> summation, so the public result and native overflow bound remain unchanged; recursive arithmetic uses u128.
Matched the C recursive cache bounds: prime-sized table and eight extra probes. Compilation is still holding the shared build lock; no network or oracle regeneration was used.
The focused run still owns Cargo's build lock. The gate is queued behind it; retain the existing native result contract and wait for concrete compiler/test results.
Gate found one clippy manual_div_ceil warning in the C triangular-number expression. Replaced it with the equivalent standard method; rerun the gate.
Focused tests passed: compat_arith 31/31 and prop_arith 19/19. The corrected gate is running; all four selected oracle functions pass without changing fixtures.
The next gate pass found two benchmark-only semicolon warnings. Added the semicolons; library checks and focused tests are clean. Restarting the gate and final offline suite.
Formatting and all-target Clippy now pass. Full offline tests are building; the gate will run the Rust 1.85 build, suite, and docs after the Cargo lock is released.
- 2026-10-17T18:45:31Z launcher: no commit for 60 s; 307 s elapsed. Files changed so far: benches/arith.rs, docs/worker-notes/wx-mpu-totients-f03.md, src/arith/dedekind_psi.rs, src/arith/inverse_totient.rs, src/arith/is_totient.rs, src/arith/mod.rs, src/arith/sumtotient.rs, tests/prop_arith.rs.
Final full-suite compilation is still running with output buffered by tools/test.sh. No new code failures have appeared; the gate remains queued for its MSRV build.
The final test build released Cargo's lock and the gate is compiling with Rust 1.85. Whitespace validation also passed; waiting for the full-suite results.
- 2026-10-17T18:47:36Z launcher: no commit for 61 s; 431 s elapsed. Files changed so far: benches/arith.rs, docs/worker-notes/wx-mpu-totients-f03.md, src/arith/dedekind_psi.rs, src/arith/inverse_totient.rs, src/arith/is_totient.rs, src/arith/mod.rs, src/arith/sumtotient.rs, tests/prop_arith.rs.
Rust 1.85 build passed. The gate is executing the full suite and has reached factor compatibility tests; all observed cases pass so far.
Gate completed with == gate: green: formatting, Clippy, Rust 1.85, full tests, 435 doctests, and documentation all passed. Final standalone tools/test.sh status is being collected.
About five minutes remain in the session budget; all requested implementation and validation work is complete.
Final: standalone tools/test.sh passed, including all 435 doctests. The gate printed == gate: green. No blocked tests, fixture edits, dependency changes, or git writes remain.
