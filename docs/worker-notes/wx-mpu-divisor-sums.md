Port the requested arithmetic functions from the local MPU 0.75 sources.
Reuse existing factoring and modular primitives and preserve the C bounds.
Keep the existing oracle fixtures, tools, crate root, and status file unchanged.
Add compatibility, algebraic, edge tests, and arithmetic benchmarks; run offline checks.
Read the C implementations and fixture shapes. Implemented the arithmetic APIs with checked native results, signed discriminants, and private Brent factor support.
Added all fifteen public functions, compatibility tests, property tests, and hot-function benchmarks. The first focused offline test is running.
Focused algebraic tests pass. Compatibility exposed Perl wrapper behavior for default exponents, tau fallback, signed abundance overflow, and modulus zero; checking and fixing those paths.
Thirteen compatibility tests now pass. Remaining work is tau values beyond the C table and the documented signed-input skip for oversized discriminants; abundance now matches the XS signed-word conversion.
