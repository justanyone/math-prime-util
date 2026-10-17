Reassess the existing native modular functions against the MPU references.
Move each public function into its own file, as required by this slice.
Keep shared helpers private and preserve the existing fixtures and tooling.
Run focused compatibility and property tests, then the offline gate.
Split the four APIs into separate modules; aligned the trial limit (20,000) and factorial composite bound with the C reference.
Focused offline tests are running; added exact binomial and direct factorial checks for the split modules.
Module extraction is complete; offline cargo checks are still pending, with no diagnostics emitted yet.
The gate passed formatting and is waiting for the focused cargo test build lock; both required Rust toolchains are installed.
Restored the C factorial fast path for n <= 10; fixture and property tests retain signed/bigint skip comments and concurrency checks.
At four minutes elapsed, focused checks are still building. No code diagnostics have been reported; the gate remains queued behind Cargo.
Implementation review is complete. The remaining work is to observe the focused tests and offline gate; Cargo has not emitted a compiler error.
All four oracle tests and all six property/concurrency tests passed offline. Removed the unused import found during compilation.
- 2026-10-17T18:47:41Z launcher: no commit for 60 s; 395 s elapsed. Files changed so far: docs/worker-notes/wx-mpu-znlog-f01.md, src/modular/binomialmod.rs, src/modular/factorialmod.rs, src/modular/mod.rs, src/modular/native.rs, src/modular/pisano_period.rs, src/modular/znlog.rs.
Compatibility and property checks are green. Modular unit tests and the full gate are still running offline; no dependencies or fixtures changed.
All nine selected modular unit tests passed, including the new exact arithmetic checks. The gate is now running Clippy on the corrected source.
The required gate stopped in Clippy on four float_cmp errors in existing count/proving tests outside this task. The modular checks passed; keep those sibling files unchanged.
