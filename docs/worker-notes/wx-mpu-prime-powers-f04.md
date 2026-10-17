Port four unsigned perfect-power functions from MPU perfect_powers.c.
Keep the existing fixtures, tools, crate root, and STATUS.md unchanged.
Use the existing integer primitives and add compat, property, and benchmark coverage.
Run offline focused tests and the repository gate; record blockers here.
Read the C algorithms and existing integer helpers. Fixtures include signed, bigint, and range-count cases outside the unsigned scalar API; compat tests will name these exclusions.
Implemented the four functions, fixture checks, algebraic and boundary tests, and benchmarks. Focused offline tests are running; reviewing full-width exponent behavior next.
Focused oracle and algebraic tests passed. An added exhaustive binary-exponent regression failed at exponent 41; restored MPU powerof_ret's power-of-two fast path locally without changing the shared integer module.
All four oracle tests and seven property/boundary tests now pass offline, including exponent 41 and concurrent/repeated calls. The full repository gate is running.
