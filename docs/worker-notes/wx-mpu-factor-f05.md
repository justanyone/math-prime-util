Port the four factor predicates from MPU 0.75 in separate source files.
Keep shared module edits minimal; leave generated status and fixtures unchanged.
Use existing offline fixtures and add algebraic property tests and benchmarks.
Run focused tests and the repository gate; record any unavailable dependencies.
The factor module has no prerequisite factoring API; predicate fallbacks will use private support and retain the C filters and bounds.
Implemented predicates, private C rho/trial support, oracle tests, independent counts, concurrency checks, and benchmarks; running focused tests next.
Focused compile found Option-returning modular helpers; corrected handling for the nonzero internal modulus and rerunning.
