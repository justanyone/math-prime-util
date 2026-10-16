Implement the sieve API and offline compatibility tests within the named paths.
PLAN sections 5–8 are read. The five named C reference files are absent.
The later task rules prohibit edits to tools, fixtures, STATUS.md, and src/lib.rs.
Inspect the existing Rust and oracle data before selecting the feasible work.
A faithful C algorithm port cannot be verified without the reference submodule.
The reference submodule is absent. Existing fixtures are available; implement bounded-memory sieving and record the unverified C fidelity.
Implemented API fallbacks and tests. The sieve uses segmented marking, not yet the required C wheel tables. Prime sums use a quotient table for wide intervals.
Focused compatibility and property tests launched offline. Next check formatting and the required gate; all prohibited paths remain untouched.
Formatting completed. Focused tests and gate are still running; the gate waits on the Cargo build lock. Exact C port remains blocked by absent sources.
Gate found numeric separator lints and an infallible segment-size conversion doc lint. Corrected both; also widened the prime-sum triangular-number addition before adding one.
All eight compatibility tests and four property/regression tests passed offline. Added direct cache state tests for duplicate requests, growth, reset, retained snapshots, and Send + Sync.
