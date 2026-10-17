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
