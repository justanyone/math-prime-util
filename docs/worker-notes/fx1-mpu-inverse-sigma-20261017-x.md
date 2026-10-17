# fx1-mpu-inverse-sigma worker notes
- Plan: address reviewer findings 1-4 from docs/reviews/rev-mpu-inverse-sigma-1.md (commit d8f33bd): k=0 returns Some(0), inverse_interpolate port, restore compat/prop test files, k=1 fast paths.
- Fix each with a test that fails without the fix.
