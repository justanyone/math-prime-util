Verdict: FAIL

1. tests/compat_int.rs:1 — Open. Replacing this file removes the existing integer oracle tests (sqrtint, roots, powers, logarithms, gcd/lcm/gcdext and modular operations). tests/prop_int.rs:1 likewise removes their randomized algebraic checks. Integer regressions can now pass the reduced suite. Preserve those tests and add configuration tests separately.

2. src/config/mod.rs:118 — Open. The maxbits branch accepts prime_set_config(&[("maxbits", 1)]) and reports success. MPU's prime_set_config in reference/mpu/lib/Math/Prime/Util.pm:277 has no maxbits setter and rejects that key. The new prop_int_maxbits_stays_in_native_width test explicitly enshrines this incompatible success path; replace it with an oracle rejection test.

3. src/config/mod.rs:128 — Open. Matching only assume_rh rejects supported MPU spellings such as rh, grh, erh and assume rh. The reference setter accepts these through its RH patterns. Add compatibility cases that fail with the current match and verify the resulting assume_rh state.

4. tests/compat_int.rs:3 — Open. This new configuration family substitutes hard-coded assertions for the required generated oracle fixtures. The complete digest includes neither a tools/gen_fixtures.pl generator section nor a committed configuration TSV. Generate reference cases, including accepted aliases and rejected keys, and consume them in compatibility tests.

The stopped run is incomplete against the acceptance criteria; the diff does not establish completion. There were no earlier findings to verify. The update stages changes before committing under one write lock, so validation errors do not partially apply entries; duplicate entries are processed in slice order and all loops are bounded by their input slices. Platform constants match the documented fixed Rust numeric types and the XS decimal-mantissa formula.

Validation: the supplied digest is the candidate; this checkout is the base. Local test/gate outcomes on this checkout cannot establish that the candidate passes. No implementation or test files were modified during review.

Base-only checks: tools/test.sh --test compat_int passed all 12 existing oracle tests; tools/test.sh passed; tools/gate.sh printed == gate: green. The initial name-filtered compat_int invocation selected zero tests, so it was followed by the explicit test-target invocation. These checks do not validate candidate code.
