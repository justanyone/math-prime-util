Port four unsigned perfect-power functions from MPU perfect_powers.c.
Keep the existing fixtures, tools, crate root, and STATUS.md unchanged.
Use the existing integer primitives and add compat, property, and benchmark coverage.
Run offline focused tests and the repository gate; record blockers here.
Read the C algorithms and existing integer helpers. Fixtures include signed, bigint, and range-count cases outside the unsigned scalar API; compat tests will name these exclusions.
