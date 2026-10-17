Reassess the existing native modular functions against the MPU references.
Move each public function into its own file, as required by this slice.
Keep shared helpers private and preserve the existing fixtures and tooling.
Run focused compatibility and property tests, then the offline gate.
Split the four APIs into separate modules; aligned the trial limit (20,000) and factorial composite bound with the C reference.
Focused offline tests are running; added exact binomial and direct factorial checks for the split modules.
Module extraction is complete; offline cargo checks are still pending, with no diagnostics emitted yet.
The gate passed formatting and is waiting for the focused cargo test build lock; both required Rust toolchains are installed.
Restored the C factorial fast path for n <= 10; fixture and property tests retain signed/bigint skip comments and concurrency checks.
