Port the requested arithmetic functions from the local MPU 0.75 sources.
Reuse existing factoring and modular primitives and preserve the C bounds.
Keep the existing oracle fixtures, tools, crate root, and status file unchanged.
Add compatibility, algebraic, edge tests, and arithmetic benchmarks; run offline checks.
Read the C implementations and fixture shapes. Implemented the arithmetic APIs with checked native results, signed discriminants, and private Brent factor support.
