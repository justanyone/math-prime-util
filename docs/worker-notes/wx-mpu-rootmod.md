Port the rootmod functions after checking the existing modular API and C algorithms.
Keep all changes within the task paths and use existing arithmetic dependencies.
The task forbids fixture and tools changes; use the existing modular oracle cases.
Run focused offline tests, then the required gate, and record any remaining gaps.
The factoring API is absent. The fixture includes signed and bigint cases outside the native unsigned API; compatibility tests will name these skips.
Implemented prime square roots, prime-power lifting, CRT, all square roots, modular division, and Cornacchia. Next: general roots and fixture checks.
General roots now use split-k Tonelli-Shanks and Hensel/CRT. All roots split k into primes; exceptional prime-power roots currently use digit lifting rather than the C recursive shortcut.
Focused compatibility and property tests are running offline. Added repeated-call, concurrent-call, zero-modulus, and root-count-limit regressions plus hot-function benchmarks.
The first focused run found two issues: empty root lists map to fixture undef, and prime-power square-root recursion must preserve unreduced a. Fixed both. Clippy found one midpoint warning; replaced the expression without changing its valu <<cut 2 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed -21 bytes)
All six native fixture tests passed. Replaced exceptional digit lifting with MPU's recursive _allrootmod_prime_power algorithm; rerunning the same tests before the final gate.
The C recursive all-root path also passes native fixtures and properties. Added word-boundary and root-existence checks. Factoring still uses a private trial-division fallback because this checkout has no factorint API; this limits performa <<cut 23 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed -1 bytes)
Word-boundary tests and exhaustive small root-existence checks passed. Restored the C small-modulus table and fixed a transcription error caught by the oracle. The final gate is running offline.
Gate reached Clippy and found three pre-existing float_cmp errors in src/count/mod.rs:240-242, outside the authorized paths. Fixed the benchmark semicolon warning in scope. Will leave count untouched and report the gate blocker.
Final focused result: all six native compatibility tests and nine property/regression tests pass offline. Signed and bigint fixture inputs are explicitly skipped. The full offline test wrapper and final gate have been launched; no tools, fi <<cut 76 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 52 bytes)
