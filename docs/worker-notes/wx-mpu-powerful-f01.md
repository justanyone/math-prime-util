Reassess the existing powerful implementation against MPU 0.75.
Split the four requested exports into individual files and preserve sibling APIs.
Add focused oracle and algebraic regression coverage without changing fixtures or tools.
Run focused offline tests and the repository gate; record any blockers here.
The four exports now have separate files. Restored C quick rejection and count fast paths; retained shared sibling helpers.
Added independent factor-exponent, prefix, range-threshold, native-edge, and parallel repeat-call tests; added membership benchmark.
Focused tests are running offline. Oracle inspection confirms k=0 uses the public k=1 sum convention; unsigned APIs skip only signed and oversized arguments.
The gate reached clippy and is waiting for the focused build lock. Added exact signed-case skip comments and documented the public zero-exponent sum behavior.
All requested code and regression tests are written. Waiting for offline validation; no fixture, tooling, dependency, or STATUS changes are needed.
- 2026-10-17T18:48:02Z launcher: no commit for 62 s; 246 s elapsed. Files changed so far: benches/families.rs, docs/worker-notes/wx-mpu-powerful-f01.md, src/families/is_powerful.rs, src/families/mod.rs, src/families/powerful.rs, src/families/powerful_count.rs, src/families/powerful_numbers.rs, src/families/sumpowerful.rs, tests/compat_families.rs, tests/prop_families.rs.
Offline validation is still blocked on the shared build directory lock. Trying an isolated target directory so validation can proceed independently.
The isolated offline test build is running; shared-target validation remains queued. No code failures have been reported yet.
The isolated gate has also started and will validate formatting, clippy, MSRV, tests, and docs after the focused build releases its lock.
Family-wide validation has not returned. Starting a powerful-only oracle filter in a separate target to isolate this task from sibling test runtime.
Powerful-only oracle validation is running offline. The full gate remains pending; five-minute remaining checkpoint is approaching.
About five minutes remain. Code, docs, benchmarks, oracle skip comments, and algebraic regressions are complete; formatting passed, while cargo validation is still pending behind builds.
Validation remains pending with no compiler or test failure output. Continuing to wait for the focused test results.
