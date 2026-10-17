Port the four modular functions from the MPU C sources.
Reuse existing modular primitives and preserve the oracle API semantics.
Add compatibility, algebraic tests, and benchmarks without changing fixtures or tools.
Validate offline with focused tests and the repository gate.
Read the C paths and found the factoring module is a placeholder; implement a private deterministic helper for these ports.
Implemented factorial, binomial prime-power/Granville CRT, interleaved BSGS, Pohlig-Hellman, non-coprime reduction, and Pisano prime-power search; running oracle tests next.
Native oracle tests pass. Properties found one incorrect expected value in a new test (640, not 0); correcting it and resolving lint before the gate.
Focused compatibility and property tests pass. Added the C rho fallback and segmented factorial prime grouping to retain the reference algorithm paths.
All native fixtures and six property/edge tests pass; fixed reduced rho restart state. Lint now only reports benchmark style, which is corrected. Added direct BSGS/rho and Granville branch tests.
Final validation phase: compatibility, properties, and direct BSGS/rho checks pass. Gate lint fixes are complete; checking MSRV and all offline tests next.
Gate passed formatting, pedantic Clippy, and MSRV 1.85; the full offline suite is running. Native unsigned fixture exclusions are documented in compat_modular.rs.
About five minutes remain in the session budget. Implementation and focused tests are complete; waiting for the offline full suite and gate results.
The full gate and required whole-suite run are still in existing count tests; focused modular tests remain green. Coverage was not measured locally.
