Continuation: inspect the existing implementation and compare remaining paths with MPU 0.75.
Resolve the prime_count_upper native-limit mismatch and verify k >= 5 nth-almost-prime search against generated oracle fixtures.
Run focused tests, the required gate, and available coverage measurement; record concrete blockers and results.
Do not mark READY, change dependencies, or write git metadata.
Reviewed C nth_almost_prime: Rust omits the explicit upper bound. Existing count compatibility skips native-limit Li precision errors; investigate precision without adding dependencies.
Cached the C native endpoint to avoid f64 rounding and removed both endpoint skips. Matched the C explicit-upper-bound search using existing count bounds; added oracle generator inputs for k=5..9.
Focused oracle tests and tools/gate.sh launched. Perl MPU/local::lib are absent; prohibited curl prevents the supplied bootstrap, so build a disposable copy of vendored MPU under target/ instead.
Focused nth-almost and count oracle tests pass. Gate reached full tests. Coverage cannot run: cargo-llvm-cov and LLVM tools are absent. Vendored Perl configuration succeeds but make is absent; try the installed C compiler directly.
Direct XS build also blocked by missing crypt.h. Kept existing generated C fixtures (which cover k=5 and reduced k=24); removed ungenerated fixture additions and added independent enumeration rank checks for k=5..9 instead. Existing focused <<cut 32 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 8 bytes)
Gate printed == gate: green (fmt, clippy, MSRV, tests, docs). Independent exact-rank tests for k=5..9 pass. Ticked the four implemented family functions; task remains unmarked READY. Rerun gate after the final test addition and run the requ <<cut 25 chars by kit>>

KIT NOTE, NOT FILE CONTENT: this output was shortened by the kit; the file on disk is whole; show a part with sed -n A,Bp FILE (kit removed 1 bytes)
