Port the four requested iterator functions after checking the existing APIs.
Keep sibling iterator code and generated fixtures unchanged.
Compare the MPU XS loops and factor-sieve dependencies, then add focused tests.
Run offline focused tests and the repository gate; record any blocker here.
Checked existing APIs and oracle tests; replacing per-value trial division with native factor dependencies and MPU range bounds.
Moved the four APIs to separate module files. Existing fixture coverage is retained; adding MPU shift reduction and segmented range handling.
The first extraction script failed before changing source; correcting the script and retaining the new squarefree sieve file.
Focused offline iterator tests are running. Ported the factor-sieve chunk size and root thresholds; checking grouped-factor output and extreme bounds next.
The focused build caught a module doc placement error after extraction; fixed it. Added regression tests for invalid k, clones, early breaks, nesting, and threads.
The gate is running offline. Added the unchanged 64-bit almost-prime maximum table and benchmark cases for all four workloads; generated fixtures and STATUS remain untouched.
About five minutes remain. Waiting for offline build results, then resolving gate diagnostics and checking fixture dispatch.
The gate is waiting on Cargo's build lock while focused tests compile. Corrected the k=63 boundary expectation: both 2^63 and 3*2^62 have 63 prime factors.
Build jobs have not produced final diagnostics yet. The oracle fixture tests now exercise the independent squarefree integer sieve; all requested API examples remain runnable.
Compatibility dispatch now calls all four closure APIs directly. Builds remain blocked or running without final output; checking the gate again before declaring validation status.
Added unwind/relaunch, u64 endpoint, and cube-root sieve regression cases. Waiting on the shared Cargo build lock remains the only observed validation blocker.
Started focused tests with CARGO_TARGET_DIR=target/iter-verify to avoid the shared build-directory lock. No dependencies or build scripts changed.
Final source review retained the C thresholds and maximum table, and simplified the private factor-value type for pedantic lint. Isolated focused compilation is still pending.
Verification is pending: the original gate remains blocked on Cargo's lock, while the isolated test run has not returned output. No test success is claimed.
