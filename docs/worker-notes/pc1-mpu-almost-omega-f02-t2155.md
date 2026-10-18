Continuation: inspect the existing implementation and compare remaining paths with MPU 0.75.
Resolve the prime_count_upper native-limit mismatch and verify k >= 5 nth-almost-prime search against generated oracle fixtures.
Run focused tests, the required gate, and available coverage measurement; record concrete blockers and results.
Do not mark READY, change dependencies, or write git metadata.
Reviewed C nth_almost_prime: Rust omits the explicit upper bound. Existing count compatibility skips native-limit Li precision errors; investigate precision without adding dependencies.
