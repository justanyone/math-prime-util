Port the four modular functions from the MPU C sources.
Reuse existing modular primitives and preserve the oracle API semantics.
Add compatibility, algebraic tests, and benchmarks without changing fixtures or tools.
Validate offline with focused tests and the repository gate.
Read the C paths and found the factoring module is a placeholder; implement a private deterministic helper for these ports.
Implemented factorial, binomial prime-power/Granville CRT, interleaved BSGS, Pohlig-Hellman, non-coprime reduction, and Pisano prime-power search; running oracle tests next.
