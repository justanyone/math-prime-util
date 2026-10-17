Port the native primality tests from MPU 0.75 and reuse the integer and Lucas modules.
Validate against the existing primality fixtures without changing tools or fixtures.
Keep STATUS.md unchanged as required by the task-specific instructions.
Random Miller–Rabin remains deferred until the random module is available.

Checked the existing Montgomery and Lucas APIs; starting the deterministic pseudoprime core.
Implemented Fermat, Euler, Miller–Rabin, Lucas variants, BPSW, probable, safe and Mersenne tests; preparing fixture checks.
Focused oracle and property tests started offline. Remaining work includes Frobenius, Perrin, Catalan, Chen, delicate and is_prime.
Nine oracle tests passed; fixed the three fixture adapters to supply MPU’s default base 2.
All twelve implemented-function oracle tests and four property/concurrency tests pass offline. Fixed Clippy literal separators and ilog2.
NOTES: is_prime, Perrin, Catalan, all three Frobenius tests, Chen and delicate prime tests remain unported. Benchmarks and coverage measurement remain.
NOTES: miller_rabin_random remains unimplemented pending the random module; STATUS.md is untouched under the later task instruction.
Final: tools/gate.sh printed == gate: green; fmt, pedantic Clippy, MSRV build, tests and docs passed offline. No commit or push was run.
