Port the four range loops into separate files in src/iter.
Compare the existing definitions with MPU XS.xs before changing their algorithms.
Reuse existing offline fixtures and add focused state, bounds, and closure tests.
Keep tools, fixtures, STATUS.md, and unrelated module code unchanged.
Moved the four loop APIs to separate files; semiprimes now use the existing MPU predicate and native maximum. Added bounds, stop, nesting, unwind, and thread tests.
The frozen iter fixtures already cover this family; the four fixture cases now exercise closure APIs, including the oracle callback cap. Focused tests are compiling; gate waits for the build lock.
Added a regression for an exhausted RangeInclusive input: clamping must not revive its last semiprime. Existing tools remain unchanged.
The focused offline run has not returned output yet; tools/test.sh buffers output through tail. The gate is waiting for its Cargo build lock. No test failure has been reported yet.
- 2026-10-17T18:48:10Z launcher: no commit for 60 s; 243 s elapsed. Files changed so far: benches/iter.rs, docs/worker-notes/wx-mpu-iterators-f01.md, src/iter/forcomposites.rs, src/iter/foroddcomposites.rs, src/iter/forprimes.rs, src/iter/forsemiprimes.rs, src/iter/mod.rs, tests/compat_iter.rs, tests/prop_iter.rs.
Offline checks remain pending on Cargo compilation and the shared build lock. Removed literal reversed ranges from tests to avoid a clippy warning while still checking empty inputs.
- 2026-10-17T18:49:43Z launcher: no commit for 60 s; 335 s elapsed. Files changed so far: benches/iter.rs, docs/worker-notes/wx-mpu-iterators-f01.md, src/iter/forcomposites.rs, src/iter/foroddcomposites.rs, src/iter/forprimes.rs, src/iter/forsemiprimes.rs, src/iter/mod.rs, tests/compat_iter.rs, tests/prop_iter.rs.
Focused compilation found fixture test names shadowing imported closure functions. Qualified the macro calls through math_prime_util::iter to fix the name collision.
