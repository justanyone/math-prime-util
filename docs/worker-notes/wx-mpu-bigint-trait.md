Implement the generic integer surface over u64, u128, and optional BigUint.
Read PLAN sections 5–8 and inspect the existing native APIs before editing code.
Use the MPU reference and existing fixtures without changing tools or fixtures.
Run focused offline tests and the required gate; record any environment blockers.
The native root names already exist; use Int associated methods to avoid ambiguous glob exports. Checking the explicitly allowed optional num-bigint dependency offline.
Offline test failed: num-bigint is absent from the registry cache. Remove the unresolved dependency so the checkout remains buildable; finish the native Int implementations and record BigUint as blocked.
Implemented all five Int methods for native widths, with overflow-safe u128 modular powers. Added fixture replay, algebraic properties, boundary and concurrent-call tests, and benchmarks; focused validation starts now.
Focused tests and gate are running offline. The optional backend remains blocked by the missing num-bigint package; no tool, fixture, status, or crate-root file was changed.
Focused tests exposed sieve fixtures with native overflow and alternate primality-status conventions. Keep sieve replay limited to next_prime and compare its bounded native result; clippy also requires must_use on gcd.
All 11 new focused tests passed. Preserved the pre-existing primality tests and benchmarks by adding the generic cases alongside them; rerunning the gate after the must_use fix.
Gate found benchmark style warnings after registering the existing primality benchmark. Fixed explicit imports and statement semicolons; the focused generic tests remain green.
Formatting, pedantic clippy, and the Rust 1.85 build pass. The gate is now running the full suite; no separate coverage run has been performed.
Full-suite validation continues through the existing factor tests. The final diff preserves the original tests and benchmarks and passes the whitespace check.
