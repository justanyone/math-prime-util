Port the four range loops into separate files in src/iter.
Compare the existing definitions with MPU XS.xs before changing their algorithms.
Reuse existing offline fixtures and add focused state, bounds, and closure tests.
Keep tools, fixtures, STATUS.md, and unrelated module code unchanged.
Moved the four loop APIs to separate files; semiprimes now use the existing MPU predicate and native maximum. Added bounds, stop, nesting, unwind, and thread tests.
The frozen iter fixtures already cover this family; the four fixture cases now exercise closure APIs, including the oracle callback cap. Focused tests are compiling; gate waits for the build lock.
Added a regression for an exhausted RangeInclusive input: clamping must not revive its last semiprime. Existing tools remain unchanged.
The focused offline run has not returned output yet; tools/test.sh buffers output through tail. The gate is waiting for its Cargo build lock. No test failure has been reported yet.
