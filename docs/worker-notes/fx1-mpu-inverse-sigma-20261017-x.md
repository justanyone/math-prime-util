# fx1-mpu-inverse-sigma worker notes
- Plan: address reviewer findings 1-4 from docs/reviews/rev-mpu-inverse-sigma-1.md (commit d8f33bd): k=0 returns Some(0), inverse_interpolate port, restore compat/prop test files, k=1 fast paths.
- Fix each with a test that fails without the fix.
- Done: F1 count k=0 -> Some(0) + oracle test restored; F3 restored compat_arith/prop_arith from base + appended oracle tests; F4 k=1 and k>hi early returns + unit test.
- F2 declined this run: inverse_interpolate.c port and generator/prime-signature paths not written (out of the 14-minute budget); needs its own change with oracle rows.
- Tests: tools/test.sh --test compat_arith (17 pass), tools/test.sh --lib inverse_sigma0 (6 pass). mw-test wrapper failed (passes --collect-only to cargo).
